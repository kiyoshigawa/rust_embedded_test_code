#![no_std]
#![no_main]

use cortex_m as cm;
use cortex_m::Peripherals as cm_p;
use defmt_rtt as _;
use embedded_hal::prelude::_embedded_hal_blocking_delay_DelayMs;
use hal::gpio::Level;
use hal::pac::Peripherals as hal_p;
use hal::prelude::OutputPin;
use nrf52840_hal as hal;

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::info!("This should blink forever. If it doesn't, you broke something.");
    // Take ownership of the device peripherals
    let hal_peripherals = hal_p::take().unwrap();
    let cortex_m_peripherals = cm_p::take().unwrap();
    //set up systick:
    let mut systick = cortex_m_peripherals.SYST;
    systick.set_clock_source(cm::peripheral::syst::SystClkSource::Core);
    systick.set_reload(1_000);
    systick.clear_current();
    systick.enable_counter();
    //set up Delay struct:
    let mut delay = hal::Delay::new(systick);
    //get PORT0 for GPIO use:
    let port0 = hal::gpio::p0::Parts::new(hal_peripherals.P0);
    //set up pin p0_04 for LED use:
    let mut led = port0.p0_04.into_push_pull_output(Level::Low);
    loop {
        if led.set_high().is_err() {
            break;
        }
        delay.delay_ms(1000_u16);
        if led.set_low().is_err() {
            break;
        }
        delay.delay_ms(1000_u16);
    }
    defmt::info!("You failed to set an LED. Good work.");
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
