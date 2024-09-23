#![no_main]
#![no_std]

use embassy_executor::Spawner;
use {defmt_rtt as _, panic_probe as _, cortex_m_rt as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let _ = embassy_nrf::init(Default::default());
    defmt::println!("Hello, world!");
}
