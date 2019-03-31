#![no_std]
#![no_main]

extern crate stm32f4;
extern crate panic_halt;

use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::{entry, exception, ExceptionFrame};
use cortex_m_semihosting::{hprint, hprintln};
use stm32f4::stm32f412::{interrupt, Interrupt, NVIC};

#[entry]
fn main() -> ! {
    let p = cortex_m::Peripherals::take().unwrap();

    let mut syst = p.SYST;
    let mut nvic = p.NVIC;

    nvic.enable(Interrupt::EXTI0);

    // configure the system timer to wrap around every second
    syst.set_clock_source(SystClkSource::Core);
    syst.set_reload(8_000_000); // 1s
    syst.enable_counter();


    loop {
        // busy wait until the timer wraps around
        while !syst.has_wrapped() {}

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
