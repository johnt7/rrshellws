use crc_crate::{Crc, CRC_16_IBM_3740};
use defmt::*;
use embassy_executor::Spawner;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::channel::{Channel, Receiver, Sender};
use embassy_time::Timer;
use heapless::Vec;

use devshell::DevShell;
use super::{NUM_SHELLS, Q_DEV_SIZE, Q_SER_SIZE};

pub struct Mux {
    in_ch: Channel<NoopRawMutex, u8, Q_SER_SIZE>,
    out_ch: Channel<NoopRawMutex, u8, Q_SER_SIZE>,
    mux_in_queues: Option<Vec<Receiver<'static, NoopRawMutex, u8, Q_DEV_SIZE>, NUM_SHELLS>>, // channels that a multiplexor uses to receive data from an endpoint
    mux_out_queues: Option<Vec<Sender<'static, NoopRawMutex, u8, Q_DEV_SIZE>, NUM_SHELLS>>, // channels that a multiplexor uses to send data to an endpoint
    crc: bool, // flag to indicate if CRC is used
}
#[derive(Debug)]
enum MuxState {
    Waiting,
    Gathering { header: MuxHeader, left: u8 },
}

// TODO make num devshells generic --- can't do it now, no generic in a task definition
// TODO make queue sizes a generic parameter
impl Mux {
    pub fn new(crc: bool) -> Mux {
        let in_ch = Channel::<NoopRawMutex, u8, Q_SER_SIZE>::new();
        let out_ch = Channel::<NoopRawMutex, u8, Q_SER_SIZE>::new();
        Mux {
            in_ch,
            out_ch,
            mux_in_queues: Some(Vec::new()),
            mux_out_queues: Some(Vec::new()),
            crc,
        }
    }

    /// get the queues that the outside world uses to send and receive data to us.
    /// Can ony be used once, afterwards gets and error because the channel can only have one receiver
    pub fn get_queues(
        &self,
    ) -> (
        Sender<NoopRawMutex, u8, Q_SER_SIZE>,
        Receiver<NoopRawMutex, u8, Q_SER_SIZE>,
    ) {
        (self.in_ch.sender(), self.out_ch.receiver())
    }

    /// add a new mux endpoint to the multiplexor
    pub fn add_endpoint(&mut self, dev_shell: &'static mut DevShell) {
        let (itx, irx) = dev_shell.get_queues();
        let res = self.mux_in_queues.as_mut().unwrap().push(irx);
        if res.is_err() {
            defmt::panic!("Mux: failed to add endpoint 1");
        }
        let res = self.mux_out_queues.as_mut().unwrap().push(itx);
        if res.is_err() {
            defmt::panic!("Mux: failed to add endpoint 2");
        }
    }

    /// function that starts up the mux tasks
    pub fn run(&'static mut self, spawner: &Spawner) {
        // pull the queus from self, so it can be passed to the threads
        let iq = self.in_ch.receiver();
        let oq = self.out_ch.sender();
        let miq = self.mux_in_queues.take().unwrap();
        let moq = self.mux_out_queues.take().unwrap();
        let crc = self.crc;
        info!("mux: starting input crc={}", crc);
        unwrap!(spawner.spawn(runi(iq, moq, crc)));
        info!("mux: starting output");
        unwrap!(spawner.spawn(runo(oq, miq, crc)));
        info!("mux: shut down input");
    }
}

/// Output handling. This is the task that reads from the mux queues and sends to the outside world.
/// It waits, then if there is data from the selected mux queue, it pulls it from the q and sends it to the serial.
/// It switches to the next queue each time through the loop.
/// The data pulled from a queue is given a header with the length and channel number.
#[embassy_executor::task]
pub async fn runo(
    out_queue: Sender<'static, NoopRawMutex, u8, Q_SER_SIZE>,
    mux_in_queues: Vec<Receiver<'static, NoopRawMutex, u8, Q_DEV_SIZE>, NUM_SHELLS>,
    crc_flag: bool,
) {
    info!("mux: runot");
    let mut buf = [0; 17];
    let crc = Crc::<u16>::new(&CRC_16_IBM_3740);
    loop {
        for channel_in_use in 0..mux_in_queues.len() as u8 {
            //                info!("mux: waiting for output");
            let mut count: u8 = 0;
            loop {
                match mux_in_queues[channel_in_use as usize].try_receive() {
                    Ok(t) => {
                        buf[count as usize] = t;
                        count += 1;
                        info!("mux: got idata {}", channel_in_use);
                        if count >= buf.len() as u8 - 2 {
                            break;
                        }
                    }
                    _ => {
                        // No more
                        break;
                    }
                }
            }
            match count {
                0 => {
                    // nothing, skip this channel
                }
                _ => {
                    if crc_flag {
                        count += 2;
                    }
                    let head_byte = MuxHeader {
                        channel: channel_in_use,
                        count: count,
                    };
                    if crc_flag {
                        let mut digest = crc.digest_with_initial(0xffff);
                        digest.update(&[head_byte.into()]);
                        digest.update(&buf[0..(count - 2) as usize]);
                        let crc_b = digest.finalize().to_le_bytes();
                        buf[count as usize - 2] = crc_b[0];
                        buf[count  as usize - 1] = crc_b[1];
                    }
                    info!("mux send head {:x}", <MuxHeader as Into<u8>>::into(head_byte));
                    out_queue.send(head_byte.into()).await;
                    for i in 0..count {
                        info!("mux send data {:x}", buf[i as usize]);
                        out_queue.send(buf[i as usize]).await;
                    }
                }
            }
            Timer::after_millis(500).await;
        }
    }
}

/// Input handling. This is the task that reads from the outside world and puts it into a mux queues.
/// the first byte is a header that contains the channel number and the count of bytes to follow.
/// It reads the header, then reads the data, then sends the data to the appropriate mux queue.
#[embassy_executor::task]
async fn runi(
    in_queue: Receiver<'static, NoopRawMutex, u8, Q_SER_SIZE>,
    mux_out_queues: Vec<Sender<'static, NoopRawMutex, u8, Q_DEV_SIZE>, NUM_SHELLS>,
    crc_flag: bool,
) {
    info!("mux: run1");
    let mut buffer = [0; 16];
    let mut mux_state = MuxState::Waiting;
    let crc = Crc::<u16>::new(&CRC_16_IBM_3740);

    loop {
        info!("mux: waiting for input");
        let data = in_queue.receive().await;
        info!("mux: got data = {:x}", data);
        match mux_state {
            MuxState::Waiting => {
                let header: MuxHeader = data.into();
                if header.channel as usize >= mux_out_queues.len() {
                    info!("Mux: invalid channel {:?}", header.channel);
                } else {
                    mux_state = MuxState::Gathering {
                        header,
                        left: header.count,
                    };
                    //                        info!("Mux: found channell {:?}", header.channel);
                }
            }
            MuxState::Gathering { header, left } => {
                buffer[(header.count - left) as usize] = data;
                if left == 1 {
                    //                        info!("Mux: left 1 {:?}", header.channel);
                    let mut count = header.count;
                    if crc_flag {
                        let mut digest = crc.digest_with_initial(0xffff);

                        info!("Mux: crc {:?}", <MuxHeader as Into<u8>>::into(header));
                        digest.update(&[header.into()]);
                        digest.update(&buffer[0..(header.count - 2) as usize]);
                        info!("Mux: data {:?}", &buffer[0..(header.count - 2) as usize]);
                        let crc = digest.finalize();

                        let crc1 = buffer[(header.count - 1) as usize];
                        let crc2 = buffer[(header.count - 2) as usize];
                        info!(
                            "Mux: crc {:x} {:x} {:x} {:x}",
                            crc1,
                            crc2,
                            crc.to_le_bytes()[0],
                            crc.to_le_bytes()[1]
                        );
                        if crc != ((crc1 as u16) << 8 | crc2 as u16) {
                            info!("Mux: crc error");
                            mux_state = MuxState::Waiting;
                            continue;
                        }
                        count -= 2;
                    }
                    for i in 0..count {
                        let data = buffer[i as usize];
                        mux_out_queues[header.channel as usize].send(data).await;
                    }
                    mux_state = MuxState::Waiting;
                //                        info!("Mux: back to waiting");
                } else {
                    mux_state = MuxState::Gathering { header, left: left - 1 };
                }
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct MuxHeader {
    pub channel: u8,
    pub count: u8,
}

impl From<u8> for MuxHeader {
    fn from(data: u8) -> Self {
        MuxHeader {
            channel: (data >> 4) & 0x0f,
            count: data & 0x0f,
        }
    }
}
impl Into<u8> for MuxHeader {
    fn into(self) -> u8 {
        (self.channel << 4) | self.count
    }
}
