#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::{entry, prelude::*, Delay, Leds};

#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, Leds) = aux5::init();
    let delay_timer = 50_u8;

    loop {
        for current in 0..8 {
            let next = (current + 1) % 8;
            
            leds[next].on();
            delay.delay_ms(delay_timer);
    
            leds[current].off();
            delay.delay_ms(delay_timer);
        }
    }
}
