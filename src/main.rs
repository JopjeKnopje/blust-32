#![deny(unsafe_code)]
#![no_std]
#![no_main]

use defmt::println;
use defmt_rtt as _;

use embedded_graphics::{
    Drawable,
    geometry::Point,
    image::{Image, ImageRaw},
    pixelcolor::BinaryColor,
};
use nb::block;

use panic_probe as _;

use cortex_m_rt::entry;
use ssd1306::{I2CDisplayInterface, Ssd1306, mode::DisplayConfig, size::DisplaySize128x64};
use stm32f1xx_hal::{
    i2c::{DutyCycle, Mode},
    pac,
    prelude::*,
    rcc,
    timer::Timer,
};

#[entry]
fn main() -> ! {
    // Get access to the core peripherals from the cortex-m crate
    let cp = cortex_m::Peripherals::take().unwrap();
    // Get access to the device specific peripherals from the peripheral access crate
    let pac = pac::Peripherals::take().unwrap();

    let mut flash = pac.FLASH.constrain();
    let mut rcc = pac.RCC.freeze(
        if 1 == 1 {
            // Freeze the configuration of all the clocks in the system and store the frozen frequencies in
            // `clocks`
            rcc::Config::hse(8.MHz())
        } else {
            // My blue pill with a stm32f103 clone dose not seem to respect rcc so will not compensate its pulse legths
            // with a faster clock like this. And so the sensor dose not have time to respond to the START pulse.
            // I would be interested if others with real stm32f103's can use this program with the faster clocks.
            rcc::Config::hse(8.MHz()).sysclk(48.MHz()).pclk1(6.MHz())
        },
        &mut flash.acr,
    );

    let gpiob = pac.GPIOB.split(&mut rcc);

    let scl = gpiob.pb6;
    let sda = gpiob.pb7;

    let i2c = pac.I2C1.blocking_i2c(
        (scl, sda),
        Mode::Fast {
            frequency: 400.kHz(),
            duty_cycle: DutyCycle::Ratio16to9,
        },
        &mut rcc,
        1000,
        10,
        1000,
        1000,
    );
    println!("done setting up i2c");

    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(
        interface,
        DisplaySize128x64,
        ssd1306::rotation::DisplayRotation::Rotate0,
    )
    .into_buffered_graphics_mode();
    println!("done setting up display");

    display.init().unwrap();
    println!("display initialized");

    let raw: ImageRaw<BinaryColor> = ImageRaw::new(include_bytes!("./rust.raw"), 64);
    let im = Image::new(&raw, Point::new(32, 0));
    im.draw(&mut display).unwrap();

    display.flush().unwrap();

    // Acquire the GPIOC peripheral
    let mut gpioc = pac.GPIOC.split(&mut rcc);

    // Configure gpio C pin 13 as a push-pull output. The `crh` register is passed to the function
    // in order to configure the port. For pins 0-7, crl should be passed instead.
    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
    // Configure the syst timer to trigger an update every second
    let mut timer = Timer::syst(cp.SYST, &rcc.clocks).counter_hz();
    timer.start(10.Hz()).unwrap();

    loop {
        block!(timer.wait()).unwrap();
        led.set_high();
        block!(timer.wait()).unwrap();
        led.set_low();
    }
}
