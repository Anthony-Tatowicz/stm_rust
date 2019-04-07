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
    syst.clear_current();
    syst.enable_counter();


    let peripherals = stm32f4::stm32f412::Peripherals::take().unwrap();
    let gpiog = &peripherals.GPIOE;
    let rcc = &peripherals.RCC;


    rcc.ahb1enr.modify(|_, w| w.gpioeen().enabled());
    gpiog.moder.modify(|_, w| w.moder0().output());
    gpiog.otyper.modify(|_, w| w.ot0().clear_bit());
    gpiog.bsrr.write(|w| w.bs0().clear_bit());

    rcc.ahb1enr.modify(|_, w| w.gpioeen().enabled());
    gpiog.moder.modify(|_, w| w.moder1().output());
    gpiog.otyper.modify(|_, w| w.ot1().clear_bit());
    gpiog.bsrr.write(|w| w.bs1().clear_bit());

    rcc.ahb1enr.modify(|_, w| w.gpioeen().enabled());
    gpiog.moder.modify(|_, w| w.moder2().output());
    gpiog.otyper.modify(|_, w| w.ot2().clear_bit());
    gpiog.bsrr.write(|w| w.bs2().clear_bit());

    rcc.ahb1enr.modify(|_, w| w.gpioeen().enabled());
    gpiog.moder.modify(|_, w| w.moder3().output());
    gpiog.otyper.modify(|_, w| w.ot3().clear_bit());
    gpiog.bsrr.write(|w| w.bs3().clear_bit());


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
