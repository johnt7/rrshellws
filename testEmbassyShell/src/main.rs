#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::usart::{Config, Uart};
use embassy_stm32::{bind_interrupts, peripherals, usart};
use {defmt_rtt as _, panic_probe as _};

//mod devshell;
//mod dictionary;
//mod logger;
//mod mux;
//mod st7735;
//mod uart_handler;

bind_interrupts!(struct Irqs {
    UART7 => usart::InterruptHandler<peripherals::UART7>;
});

const Q_SER_SIZE: usize = 10;
const Q_DEV_SIZE: usize = 10;
const NUM_SHELLS: usize = 3;
pub const USE_CRC: bool = true;

static mut DEVSHELL1: Option<devshell::DevShell> = None;
static mut DEVSHELL2: Option<devshell::DevShell> = None;
static mut LOGGER: Option<logger::Logger> = None;
static mut MUX: Option<mux::Mux> = None;
static mut UART_HANDLER: Option<uart_handler::UartHandler> = None;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("Hello mux World!");
    let p = embassy_stm32::init(Default::default());
    unsafe {
        if LOGGER.is_none() {
            LOGGER = Some(logger::Logger::new());
        } else {
            defmt::panic!("LOGGER already initialized");
        }
        if DEVSHELL1.is_none() {
            DEVSHELL1 = Some(devshell::DevShell::new(1));
        } else {
            defmt::panic!("DEVSHELL1 already initialized");
        }
        if DEVSHELL2.is_none() {
            DEVSHELL2 = Some(devshell::DevShell::new(2));
        } else {
            defmt::panic!("DEVSHELL2 already initialized");
        }
        if MUX.is_none() {
            MUX = Some(mux::Mux::new(USE_CRC));
        } else {
            defmt::panic!("MUX already initialized");
        }
        MUX.as_mut().unwrap().add_endpoint(DEVSHELL1.as_mut().unwrap());
        MUX.as_mut().unwrap().add_endpoint(DEVSHELL2.as_mut().unwrap());
    }
    unsafe {
        let config = Config::default();
        let uart = Uart::new(p.UART7, p.PA8, p.PA15, Irqs, p.DMA1_CH0, p.DMA1_CH1, config).unwrap();
        UART_HANDLER = Some(uart_handler::UartHandler::new(uart));
        UART_HANDLER.as_mut().unwrap().add_endpoint(MUX.as_ref().unwrap());
    }
    unsafe {
        DEVSHELL1.as_mut().unwrap().run(&spawner);
        DEVSHELL2.as_mut().unwrap().run(&spawner);
        MUX.as_mut().unwrap().run(&spawner);
        UART_HANDLER.as_mut().unwrap().run(&spawner);
    }
}
