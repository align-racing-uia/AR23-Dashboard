#![no_std]
#![no_main]

use display_interface_spi::SPIInterfaceNoCS;
use embedded_graphics_core::pixelcolor::Bgr565;
use embedded_graphics_core::pixelcolor::Bgr666;
use embedded_graphics_core::pixelcolor::Rgb565;
use embedded_graphics_core::pixelcolor::Rgb666;
use embedded_hal::digital::v2::OutputPin;
use rp_pico::entry;

// Import important traits from the Hardware-Abstraction-Layer.
use rp_pico::hal::prelude::*;

// Import the Hardware Abstaction Library, and Peripheral Access Create from the Board support package.
use rp_pico::hal;
use rp_pico::hal::pac;
use rp_pico::hal::spi;
use rp_pico::hal::gpio;

// We need this package to make the program halt on a panic.
use panic_halt as _;

// Adds support for some screen drivers, including ILI9486, which is the one we are using.
use mipidsi::*;

// Adds functions to convert numbers to Hz.
use fugit::RateExtU32;

// Adds functions for display interface for SPI.
use display_interface_spi::SPIInterface;

// Add functions for embedded graphics
use embedded_graphics_core::draw_target::DrawTarget;
use embedded_graphics_core::pixelcolor::{RgbColor};


#[entry]
fn main() -> ! {

    // This can only be done once, as we take all the peripherals from the Peripheral-Access-Crate.
    let mut pac = pac::Peripherals::take().unwrap();

    // Grabbing the core peripheral from the PAC.
    let core = pac::CorePeripherals::take().unwrap();

    // Set up a watchdog, to monitor that we dont get stuck in a loop.
    // We are creating a Watchdog object by grabbing the watchdog from the pac and putting it into a object.
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    // Configure the clocks
    //
    // The default is to generate a 125 MHz system clock
    let clocks = hal::clocks::init_clocks_and_plls(
        rp_pico::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    // Create a delay source with the core system timer, and the system clock frequency.
    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    // The single-cycle I/O block controls our GPIO pins.
    let sio = hal::Sio::new(pac.SIO);

    // Create a variable containing all the pins, already mapped for us according to the Pico documentation.
    let pins = rp_pico::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS
    );

    let mut _sck_pin = pins.gpio2.into_mode::<gpio::FunctionSpi>();
    let mut _mosi_pin = pins.gpio3.into_mode::<gpio::FunctionSpi>();
    let mut cs_pin = pins.gpio5.into_push_pull_output();

    let mut led_pin = pins.led.into_push_pull_output();

    // Data/clock switch pin
    let mut dc_pin = pins.gpio6.into_push_pull_output();

    let mut rst_pin = pins.gpio7.into_push_pull_output();
    

    // Create uninitialized SPI Driver instance for the SPI0 device.
    let spi = spi::Spi::<_, _, 8>::new(pac.SPI0);

    // Initialize the SPI Driver.
    let spi = spi.init(
        &mut pac.RESETS, 
        clocks.peripheral_clock.freq(), 
        10.MHz(),
        &embedded_hal::spi::MODE_3
    );

    // Initialize display interface
    let di = SPIInterface::new(spi, dc_pin, cs_pin);


    // Create display driver
    let mut display = Builder::ili9486_rgb565(di)
        .with_display_size(480, 320)
        .with_color_order(ColorOrder::Bgr)
        .init(&mut delay, Some(rst_pin))
        .unwrap();

    delay.delay_ms(500);
    display.clear(Rgb565::RED).unwrap();

    loop {
        display.clear(Rgb565::RED).unwrap();
        delay.delay_ms(1000);
        display.clear(Rgb565::GREEN).unwrap();
        delay.delay_ms(1000);
        display.clear(Rgb565::BLUE).unwrap();

    }
}

