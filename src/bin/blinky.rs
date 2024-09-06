//! This example test the RP Pico on board LED.
//!
//! It does not work with the RP Pico W board. See wifi_blinky.rs.

#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::gpio;
use embassy_time::Timer;
use gpio::{Level, Output};
use {defmt_rtt as _, panic_probe as _};

const DOT_DURATION: u64 = 200;
const DASH_DURATION: u64 = DOT_DURATION * 3;
const SYMBOL_SPACE: u64 = DOT_DURATION;
const LETTER_SPACE: u64 = DOT_DURATION * 3;
const WORD_SPACE: u64 = DOT_DURATION * 7;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let mut led = Output::new(p.PIN_15, Level::Low);

    loop {
        send_morse_code(&mut led).await;
        Timer::after_millis(WORD_SPACE as u64).await;
    }
}

async fn send_morse_code(led: &mut Output<'_>) {
    // R: ・－・
    dot(led).await;
    dash(led).await;
    dot(led).await;
    Timer::after_millis(LETTER_SPACE).await;

    // U: ・・－
    dot(led).await;
    dot(led).await;
    dash(led).await;
    Timer::after_millis(LETTER_SPACE).await;

    // S: ・・・
    dot(led).await;
    dot(led).await;
    dot(led).await;
    Timer::after_millis(LETTER_SPACE).await;

    // T: －
    dash(led).await;
    Timer::after_millis(LETTER_SPACE).await;
}

async fn dot(led: &mut Output<'_>) {
    info!("dot");
    led.set_high();
    Timer::after_millis(DOT_DURATION).await;
    led.set_low();
    Timer::after_millis(SYMBOL_SPACE).await;
}

async fn dash(led: &mut Output<'_>) {
    info!("dash");
    led.set_high();
    Timer::after_millis(DASH_DURATION).await;
    led.set_low();
    Timer::after_millis(SYMBOL_SPACE).await;
}