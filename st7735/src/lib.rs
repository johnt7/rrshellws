#![no_std]

use defmt::*;
use embassy_stm32::gpio::Output;
use embassy_stm32::mode::Blocking;
use embassy_stm32::spi;
use embassy_time::Timer;
mod instruction;
pub use instruction::Instruction;
//use core::arch::asm;

pub struct ST7735 {
    // Field: spi
    spi: spi::Spi<'static, Blocking>,
    rdx: Output<'static>,
    led: Output<'static>,
    cs: Output<'static>,
    inverted: bool,
    rgb: bool,
    _dx: u16,
    _dy: u16,
}

pub const X_WIDTH: usize = 131;
pub const Y_WIDTH: usize = 161;

/// implement a driver for the ST7735 display
/// there are some issues around the edges.  x only seems to show 25-100
/// not sure about y
impl ST7735 {
    pub fn new(
        spi: spi::Spi<'static, Blocking>,
        rdx: Output<'static>,
        led: Output<'static>,
        cs: Output<'static>,
    ) -> Self {
        Self {
            spi,
            rdx,
            led,
            cs,
            inverted: true,
            rgb: true,
            _dx: 0,
            _dy: 0,
        }
    }

    /// Initialize the display.
    pub async fn init(&mut self) -> Result<(), ()> {
        self.led.set_low();
        self.rdx.set_low();

        self.write_command(Instruction::SWRESET, &[]).await?;
        Timer::after_millis(200).await;
        self.write_command(Instruction::SLPOUT, &[]).await?;
        Timer::after_millis(200).await;
        self.write_command(Instruction::FRMCTR1, &[0x01, 0x2C, 0x2D]).await?;
        self.write_command(Instruction::FRMCTR2, &[0x01, 0x2C, 0x2D]).await?;
        self.write_command(Instruction::FRMCTR3, &[0x01, 0x2C, 0x2D, 0x01, 0x2C, 0x2D])
            .await?;
        self.write_command(Instruction::INVCTR, &[0x07]).await?;
        self.write_command(Instruction::PWCTR1, &[0xA2, 0x02, 0x84]).await?;
        self.write_command(Instruction::PWCTR2, &[0xC5]).await?;
        self.write_command(Instruction::PWCTR3, &[0x0A, 0x00]).await?;
        self.write_command(Instruction::PWCTR4, &[0x8A, 0x2A]).await?;
        self.write_command(Instruction::PWCTR5, &[0x8A, 0xEE]).await?;
        if self.inverted {
            self.write_command(Instruction::INVON, &[]).await?; // set to inverted
        } else {
            self.write_command(Instruction::INVOFF, &[]).await?; // set to normal
        }
        if self.rgb {
            self.write_command(Instruction::MADCTL, &[0x00]).await?; // set to BGR
        } else {
            self.write_command(Instruction::MADCTL, &[0x08]).await?; // set to RGB
        }
        self.write_command(Instruction::COLMOD, &[0x05]).await?; // set to 16 bit color
        self.write_command(Instruction::DISPON, &[]).await?;
        self.led.set_low();
        Timer::after_millis(200).await;
        Ok(())
    }

    /// Clear the display and the CASET and RASET registers.
    pub async fn clear(&mut self) -> Result<(), ()> {
        let dat: [u8; X_WIDTH * 2] = [0x00; X_WIDTH * 2];
        self.set_address_window(0, 0, X_WIDTH as u16, Y_WIDTH as u16)
            .await
            .unwrap();
        self.write_command(Instruction::RAMWR, &dat).await?;
        self.cs.set_low();
        self.rdx.set_high();
        for _ in 0..Y_WIDTH {
            self.send_data(&dat).await?;
        }
        self.cs.set_high();
        info!("cleared");
        Ok(())
    }

    /// Write a command to the display
    pub async fn write_command(&mut self, command: instruction::Instruction, params: &[u8]) -> Result<(), ()> {
        self.cs.set_low();
        self.rdx.set_low();
        self.spi.blocking_write(&[command as u8]).map_err(|_| ())?;
        if !params.is_empty() {
            self.rdx.set_high();
            self.send_data(params).await.map_err(|_| ())?;
        }
        self.cs.set_high();
        Ok(())
    }

    /// writes additonal data to existing command.
    pub async fn write_data(&mut self, data: &[u8]) -> Result<(), ()> {
        self.cs.set_low();
        self.rdx.set_high();
        self.send_data(data).await.map_err(|_| ())?;
        self.cs.set_high();
        Ok(())
    }

    /// Set the address window
    pub async fn set_address_window(&mut self, sx: u16, sy: u16, ex: u16, ey: u16) -> Result<(), ()> {
        self.cs.set_low();

        self.rdx.set_low();
        self.spi.blocking_write(&[Instruction::CASET as u8]).map_err(|_| ())?;
        self.rdx.set_high();
        self.send_word(sx).await?;
        self.send_word(ex).await?;

        self.rdx.set_low();
        self.spi.blocking_write(&[Instruction::RASET as u8]).map_err(|_| ())?;
        self.rdx.set_high();
        self.send_word(sy).await?;
        self.send_word(ey).await?;

        self.cs.set_high();
        Ok(())
    }

    pub async fn write_pixel(&mut self, x: u16, y: u16, color: u16) -> Result<(), ()> {
        self.set_address_window(x, y, x, y).await?;
        self.write_command(Instruction::RAMWR, &color.to_be_bytes()).await?;
        Ok(())
    }

    pub async fn write_pixels(&mut self, sx: u16, sy: u16, ex: u16, ey: u16, colors: &[u16]) -> Result<(), ()> {
        self.set_address_window(sx, sy, ex, ey).await?;

        self.write_command(Instruction::RAMWR, &[]).await?;
        for color in colors {
            self.write_data(&color.to_be_bytes()).await?;
        }
        Ok(())
    }

    pub async fn write_pixels_color(&mut self, sx: u16, sy: u16, ex: u16, ey: u16, color: u16) -> Result<(), ()> {
        self.set_address_window(sx, sy, ex, ey).await?;
        let b_color = color.to_be_bytes();

        self.write_command(Instruction::RAMWR, &[]).await?;
        for _ in 0..((ex - sx) * (ey - sy)) {
            self.write_data(&b_color).await?;
        }
        Ok(())
    }

    async fn send_word(&mut self, value: u16) -> Result<(), ()> {
        self.send_data(&value.to_be_bytes()).await
    }

    async fn send_data(&mut self, data: &[u8]) -> Result<(), ()> {
        self.spi.blocking_write(data).map_err(|_| ())?;
        Ok(())
    }
}
