#![no_main]
#![no_std]

use defmt_rtt as _;
use nrf52840_hal as _;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    defmt::error!("panic!");
    exit();
}

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::println!("Hello, world!");

    exit();
}

fn exit() -> ! {
    cortex_m::interrupt::disable();
    loop {
        cortex_m::asm::wfi()
    }
}
