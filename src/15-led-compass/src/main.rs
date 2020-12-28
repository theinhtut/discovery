#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux15::{entry, iprint, iprintln, prelude::*};

#[entry]
fn main() -> ! {
    let (_leds, mut data, mut delay, mut itm) = aux15::init_lsm303agr();

    loop {
        iprintln!(&mut itm.stim[0], "x:{:?}, y:{:?}, z:{:?}", data.x, data.y, data.z);
        delay.delay_ms(1_000_u16);
    }
}
