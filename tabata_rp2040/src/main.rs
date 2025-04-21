#![no_std]
#![no_main]

use core::cell::RefCell;

use embassy_embedded_hal::shared_bus::blocking::spi::SpiDeviceWithConfig;
use embassy_executor::Spawner;
use embassy_rp::{gpio, spi};
use embassy_sync::blocking_mutex::{Mutex, raw::NoopRawMutex};
use embassy_time::{Delay, Timer};
use gpio::{Input, Level, Output};
use mipidsi::interface::SpiInterface;
use mipidsi::{
    Builder,
    models::ST7789,
    options::{ColorInversion, Orientation, Rotation},
};
use panic_halt as _;
use rotary_encoder_embedded::RotaryEncoder;

use tabata_core::{TabataApp, TabataInput, button::Button, update_display};

const TIMER_STEP_MS: u64 = 100;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let mut status_led = Output::new(p.PIN_13, Level::Low);

    let rotary_dt = Input::new(p.PIN_9, gpio::Pull::Up);
    let rotary_clk = Input::new(p.PIN_10, gpio::Pull::Up);
    let rotary_button = Input::new(p.PIN_8, gpio::Pull::None);
    // Initialize the rotary encoder
    let mut rotary_encoder = RotaryEncoder::new(rotary_dt, rotary_clk).into_angular_velocity_mode();
    let mut button = Button::new(1000);

    let tft_spi_clk = p.PIN_18;
    let tft_spi_mosi = p.PIN_19;
    let tft_spi_miso = p.PIN_20;
    let tft_chip_select = Output::new(p.PIN_26, Level::High);
    let tft_reset = Output::new(p.PIN_27, Level::Low);
    let tft_command = Output::new(p.PIN_28, Level::Low);

    let mut config = spi::Config::default();
    config.frequency = 64_000_000;
    config.phase = spi::Phase::CaptureOnSecondTransition;
    config.polarity = spi::Polarity::IdleHigh;

    let spi = spi::Spi::new_blocking(
        p.SPI0,
        tft_spi_clk,
        tft_spi_mosi,
        tft_spi_miso,
        config.clone(),
    );
    let spi: Mutex<NoopRawMutex, _> = Mutex::new(RefCell::new(spi));
    let spi = SpiDeviceWithConfig::new(&spi, tft_chip_select, config);

    let mut buffer = [0_u8; 512];
    let tft_spi_interface = SpiInterface::new(spi, tft_command, &mut buffer);

    let mut display = Builder::new(ST7789, tft_spi_interface)
        .display_size(240, 320)
        .reset_pin(tft_reset)
        .invert_colors(ColorInversion::Inverted)
        .orientation(Orientation::new().rotate(Rotation::Deg90))
        .init(&mut Delay)
        .unwrap();

    let mut tabata_app: TabataApp = Default::default();

    let mut current_time: u64 = 0;
    loop {
        current_time = current_time + TIMER_STEP_MS;

        rotary_encoder.update(current_time);

        let mut input: TabataInput = Default::default();
        input.button_press = button.update(TIMER_STEP_MS, rotary_button.is_high());
        input.steps = rotary_encoder.velocity() as i32;

        let _ = update_display(&mut display, &tabata_app);
        tabata_app.update(TIMER_STEP_MS, &input);

        status_led.toggle();
        Timer::after_millis(TIMER_STEP_MS).await;
    }
}
