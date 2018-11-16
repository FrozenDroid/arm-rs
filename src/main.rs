//#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

// panic-handler crate
extern crate panic_semihosting;
//#[macro_use] extern crate cortex_m;

extern crate rtfm;

//use stm32l432xx_hal;

use rtfm::app;

//use rtfm::{P0, T0, TMax};

//extern crate stm32l4;/**/


use cortex_m::asm::delay;
//use stm32l4;

extern crate stm32l4;

#[macro_use] extern crate cortex_m_semihosting;

#[app(device = stm32l4::stm32l4x2)]
const APP: () = {


    #[init]
    fn init() {
        let per: stm32l4::stm32l4x2::Peripherals = device;
        let rcc: &stm32l4::stm32l4x2::RCC = &per.RCC;
        per.LPUART1
        rcc.ahb2enr.write(|w| {
           w.gpioben().set_bit()
        });
        unsafe {
            rcc.cfgr.write(|w| {
                w.sw().bits(0b01)
            });
        }
        let gpiob = &per.GPIOB;
        gpiob.pupdr.write(|w| {
            w.pupdr3().pull_down()
        });
        gpiob.moder.write(|w| {
            w.moder3().output()
        });
//        gpiob.odr.write(|w| {
//            w.odr3().high()
//        });
        hprintln!("aa").unwrap();
        loop {

            hprintln!("{:?}", rcc.cfgr.read().sw().bits()).unwrap();
//            hprintln!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!").unwrap();
            gpiob.odr.write(|w| {
                w.odr3().bit(gpiob.odr.read().odr3().bit_is_clear())
            });
            delay(1000000);
//            hprintln!("!").unwrap();
        }

    }
};
