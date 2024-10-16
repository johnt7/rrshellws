#![no_std]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::mode::Async;
//use embassy_stm32::pac::metadata::crc;
use embassy_stm32::usart::{Uart, UartRx, UartTx};
//use embassy_stm32::mode::Async;
use embassy_sync::{
    blocking_mutex::raw::NoopRawMutex,
    channel::{Receiver, Sender},
};

use mux::Mux;
//use super::{Q_SER_SIZE, USE_CRC};
const Q_SER_SIZE: usize = 10;
// TODO centralize this
pub const USE_CRC: bool = true;

pub struct UartHandler<'a> {
    uart: Option<Uart<'a, Async>>,
    out_queue: Option<Sender<'static, NoopRawMutex, u8, Q_SER_SIZE>>,
    in_queue: Option<Receiver<'static, NoopRawMutex, u8, Q_SER_SIZE>>,
}

impl UartHandler<'static> {
    pub fn new(uart: Uart<'static, Async>) -> Self {
        UartHandler {
            uart: Some(uart),
            out_queue: None,
            in_queue: None,
        }
    }

    pub fn add_endpoint(&mut self, endpoint: &'static Mux) {
        let (tx, rx) = endpoint.get_queues();
        self.out_queue = Some(tx);
        self.in_queue = Some(rx);
    }

    pub fn run(&mut self, spawner: &Spawner) {
        let (tx, rx) = self.uart.take().unwrap().split();
        match (self.out_queue.as_ref(), self.in_queue.as_ref()) {
            (Some(qs), Some(qr)) => {
                unwrap!(spawner.spawn(uart_in_task(rx, qs.clone())));
                unwrap!(spawner.spawn(uart_out_task(tx, qr.clone())));
            }
            _ => {
                defmt::panic!("UartHandler: queues not set");
            }
        }
    }
}

#[embassy_executor::task]
async fn uart_in_task(mut uart_rx: UartRx<'static, Async>, input: Sender<'static, NoopRawMutex, u8, Q_SER_SIZE>) {
    let mut i: [u8; 15] = [0; 15];
    info!("Starting uart_in_task");
    loop {
        match uart_rx.read_until_idle(&mut i).await.ok() {
            Some(bl) => {
                info!("read DMA ilen={}", bl);
                for p in 0..bl {
                    // TODO: check if the queue is full for too long select!?
                    input.send(i[p]).await;
                }
                info!("wrote queue");
            }
            None => {
                info!("read DMA error");
            }
        };
    }
}

#[embassy_executor::task]
async fn uart_out_task(mut uart: UartTx<'static, Async>, output: Receiver<'static, NoopRawMutex, u8, Q_SER_SIZE>) {
    info!("Starting uart_out_task");
    let mut obuf: [u8; 8] = [7, 72, 72, 72, 13, 10, 81, 93];
    let mut len = obuf.len();
    if !USE_CRC {
        len -= 2;
        obuf[0] -= 2;
    }
    uart.write(&obuf[0..len]).await.ok();

    loop {
        let c = output.receive().await;
        info!("read Queue {:?}", c);
        uart.write(&[c]).await.ok();
        info!("wrote DMA");
    }
}
