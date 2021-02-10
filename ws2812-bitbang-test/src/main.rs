#![no_std]
#![no_main]

use cortex_m as cm;
use cortex_m::Peripherals as cm_p;
use defmt_rtt as _;
use hal::gpio::{Level, PushPull};
use hal::pac::Peripherals as hal_p;
use hal::prelude::OutputPin;
use nrf52840_hal as hal;

//this function will delay for a number of clock signals instead of converting to ms or us
fn delay_clock_cycles(cycles: u32, systick: &mut cm::peripheral::SYST) {
    let full_cycles = cycles >> 24;
    if full_cycles > 0 {
        systick.set_reload(0xffffff);
        systick.clear_current();
        systick.enable_counter();

        for _ in 0..full_cycles {
            while !systick.has_wrapped() {}
        }
    }
    if cycles > 1 {
        systick.set_reload(cycles - 1);
        systick.clear_current();
        systick.enable_counter();

        while !systick.has_wrapped() {}
    }
    systick.disable_counter();
}

//this is a struct to hold a color as three 8-bit RGB values:
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

//number of cycles for WS2812 timing:
const T0H: u32 = 22;
const T1H: u32 = 45;
const T0L: u32 = 51;
const T1L: u32 = 38;
const RESET: u32 = 3200;
const NUM_COLORS: usize = 4;
//this subtracts approximately however many cycles happen between
//calling the delay functions from the number of cycles to delay.
const TIMING_COMPENSATION_OFFSET: u32 = 2;

//this will send one byte of color data down the WS2811 IO line:
fn send_byte(
    byte: u8,
    led: &mut hal::gpio::p0::P0_04<hal::gpio::Output<PushPull>>,
    systick: &mut cm::peripheral::SYST,
) {
    for i in 0..8 {
        let current_bit = (byte.reverse_bits()) >> i & 0x01;
        match current_bit {
            0 => {
                //send low bit timing
                if led.set_high().is_err() {
                    defmt::error!("The LED was not set high.");
                }
                delay_clock_cycles(T0H - TIMING_COMPENSATION_OFFSET, systick);
                if led.set_low().is_err() {
                    defmt::error!("The LED was not set low.");
                }
                delay_clock_cycles(T0L - TIMING_COMPENSATION_OFFSET, systick);
            }
            1 => {
                //send high bit timing
                if led.set_high().is_err() {
                    defmt::error!("The LED was not set high.");
                }
                delay_clock_cycles(T1H - TIMING_COMPENSATION_OFFSET, systick);
                if led.set_low().is_err() {
                    defmt::error!("The LED was not set low.");
                }
                delay_clock_cycles(T1L - TIMING_COMPENSATION_OFFSET, systick);
            }
            _ => {
                defmt::error!("You managed to get a value other than 0 or 1 when doing a bitwise & with 0x01. WTF.");
            }
        }
    }
}

//this will send one Color (24-bits of color data) down the WS2811 IO line:
fn send_color(
    color: &Color,
    led: &mut hal::gpio::p0::P0_04<hal::gpio::Output<PushPull>>,
    systick: &mut cm::peripheral::SYST,
) {
    send_byte(color.g, led, systick);
    send_byte(color.r, led, systick);
    send_byte(color.b, led, systick);
}

//this will send an array of Colors down the WS2811 IO line and then send the reset signal:
fn send_color_array(
    color_array: &[Color; NUM_COLORS],
    led: &mut hal::gpio::p0::P0_04<hal::gpio::Output<PushPull>>,
    systick: &mut cm::peripheral::SYST,
) {
    //send all colors in order
    for c in color_array {
        send_color(c, led, systick);
    }
    //wait for reset when done
    delay_clock_cycles(RESET, systick);
}

#[cortex_m_rt::entry]
fn main() -> ! {
    let color_array: [Color; NUM_COLORS] = [
        Color {
            r: 255,
            g: 000,
            b: 000,
        },
        Color {
            r: 000,
            g: 255,
            b: 000,
        },
        Color {
            r: 000,
            g: 000,
            b: 255,
        },
        Color {
            r: 255,
            g: 255,
            b: 255,
        },
    ];

    defmt::info!("This should send colors to a WS2812 LED strip.");
    defmt::info!(
        "You may need to convert from 3v3 to 5v signal if it doesn't work when directly connected."
    );
    // Take ownership of the device peripherals
    let hal_peripherals = hal_p::take().unwrap();
    let cortex_m_peripherals = cm_p::take().unwrap();
    //set up systick:
    let mut systick = cortex_m_peripherals.SYST;
    systick.set_clock_source(cm::peripheral::syst::SystClkSource::External);
    systick.clear_current();
    systick.enable_counter();
    //get PORT0 for GPIO use:
    let port0 = hal::gpio::p0::Parts::new(hal_peripherals.P0);
    //set up pin p0_03 for WS2812 use:
    let mut led = port0.p0_04.into_push_pull_output(Level::Low);
    loop {
        send_color_array(&color_array, &mut led, &mut systick);
        delay_clock_cycles(64_000_000, &mut systick);
    }
    defmt::info!("You got out of the loop, good work?");
    exit()
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    defmt::error!("panicked");
    exit()
}

pub fn exit() -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}
