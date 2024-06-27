#![no_std]
#![no_main]

//use defmt::info;
use embassy_executor::Spawner;
//use embassy_stm32::Config;
use embassy_stm32 as _;
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

use cortex_m_semihosting::hprintln;

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    //let config = Config::default();
    //let _p = embassy_stm32::init(config);

    loop {
        hprintln!("Hello World!").unwrap();
        Timer::after_secs(1).await;
    }
}
