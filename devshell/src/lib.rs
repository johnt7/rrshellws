#![no_std]

use defmt::*;
use embassy_executor::Spawner;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::channel::{Channel, Receiver, Sender};
use heapless::Vec;

//use super::Q_DEV_SIZE;
const Q_DEV_SIZE: usize = 10;

const PROMPT : [u8;4] = ['\r' as u8, 'R' as u8, 'R' as u8, '_' as u8];

pub struct DevShell {
    _ct: u8,
    in_queue: Channel<NoopRawMutex, u8, Q_DEV_SIZE>,
    out_queue: Channel<NoopRawMutex, u8, Q_DEV_SIZE>,
}

struct InterpreterState<'a> {
    buffer: Vec<u8, 128>,
    echoing: bool,
    prompt: Option<&'a [u8]>,
}
impl InterpreterState<'static> {
    pub fn new() -> InterpreterState<'static> {
        InterpreterState {
            buffer: Vec::<u8, 128>::new(),
            echoing: false,
            prompt : Some(&PROMPT),
        }
    }
}
impl<'a> DevShell {
    pub fn new(ct: u8) -> DevShell {
        let in_queue = Channel::<NoopRawMutex, u8, Q_DEV_SIZE>::new();
        let out_queue = Channel::<NoopRawMutex, u8, Q_DEV_SIZE>::new();
        DevShell {
            _ct: ct,
            in_queue,
            out_queue,
        }
    }

    // TODO, make a trait out of this
    pub fn get_queues(
        &mut self,
    ) -> (
        Sender<NoopRawMutex, u8, Q_DEV_SIZE>,
        Receiver<NoopRawMutex, u8, Q_DEV_SIZE>,
    ) {
        (self.in_queue.sender().clone(), self.out_queue.receiver().clone())
    }

    pub fn run(&'static mut self, spawner: &Spawner) {
        unwrap!(spawner.spawn(devshell_task(
            self.in_queue.receiver().clone(),
            self.out_queue.sender().clone()
        )));
        // start a task to read from input and tokenize to a queue
        // start a task to write to output
    }
}

#[embassy_executor::task(pool_size = 2)]
async fn devshell_task(
    incoming: Receiver<'static, NoopRawMutex, u8, Q_DEV_SIZE>,
    outgoing: Sender<'static, NoopRawMutex, u8, Q_DEV_SIZE>,
) {
    let mut state = InterpreterState::new();
        loop {
       'shell: loop {
            // init everything
            'interpreter: loop {
                // init intepreter
                state.buffer.clear();
                if let Some(prompt) = state.prompt {
                    for c in prompt.iter() {
                        outgoing.send(*c).await;
                    }
                }
                loop {
                    // getting data
                    let data = incoming.receive().await;
                    if state.echoing {
                        outgoing.send(data).await;
                        continue;
                    }
                    if data == '\r' as u8 || data == '\n' as u8 {
                        break;
                    }
                    if let Err(_) = state.buffer.push(data) {
                        // buffer ful, emit error info
                        break 'shell;
                    }
                }
                let mut iter = state.buffer.split(|c| *c  == '\r' as u8 || *c == '\n' as u8);
                while let Some(token) = iter.next() {
                   if token.len() == 0 {
                       continue;
                   }
                   // do first token stuff
                }
                while let Some(token) = iter.next() {
                    if token.len() == 0 {
                        continue;
                    }
                    // do rest of the tokens
                }
            // split into tokens
            // check first
            // do rest

            }

        }
        // info!("devshel: process {:?}", data);
        // outgoing.send(data).await;
    }
}
