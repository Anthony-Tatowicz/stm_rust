#![no_std]
#![no_main]

extern crate stm32f4;
extern crate panic_halt;

use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::{entry, exception, ExceptionFrame};
use cortex_m_semihosting::{hprint, hprintln};
use stm32f4::stm32f412::{interrupt, Interrupt, NVIC};
use stm32f4xx_hal as hal;
use stm32f4xx_hal::prelude::*;
use stm32f4xx_hal::hal::digital::StatefulOutputPin;

#[entry]
fn main() -> ! {
    let p = cortex_m::Peripherals::take().unwrap();

    let mut syst = p.SYST;
    let mut nvic = p.NVIC;

    nvic.enable(Interrupt::EXTI0);

    // configure the system timer to wrap around every second
    syst.set_clock_source(SystClkSource::Core);
    syst.set_reload(8_000_000); // 1s
    syst.clear_current();
    syst.enable_counter();

    let perph = hal::stm32::Peripherals::take().unwrap();
    let gpioe = perph.GPIOE.split();

    let mut pe0 = gpioe.pe0.into_push_pull_output();
    let mut pe1 = gpioe.pe1.into_push_pull_output();
    let mut pe2 = gpioe.pe2.into_push_pull_output();

    pe0.set_low();
    pe1.set_low();
    pe2.set_low();


    loop {
        // busy wait until the timer wraps around
        while !syst.has_wrapped() {}

        if pe0.is_set_low() {
          pe0.set_high();
        } else {
          pe0.set_low();
        }

        // trigger the `EXTI0` interrupt
        NVIC::pend(Interrupt::EXTI0);
    }
}

#[interrupt]
fn EXTI0() {
    hprint!(".").unwrap();
}

#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    hprintln!("{:#?}", ef).unwrap();
    loop {}
}
