#![no_std]
#![no_main]

mod fmt;

use defmt::info;
use embassy_executor::Spawner;
use embassy_futures::join::join;
use embassy_stm32::gpio::{OutputOpenDrain, OutputType};
use embassy_stm32::Config;
use embassy_stm32::{
    bind_interrupts,
    gpio::{Level, Output, Speed},
    peripherals,
    time::Hertz,
};
use embassy_time::Timer;
use embassy_usb::{
    class::cdc_acm::{CdcAcmClass, State},
    driver::EndpointError,
    Builder,
};
#[cfg(not(feature = "defmt"))]
use panic_halt as _;
use panic_probe as _;
#[cfg(feature = "defmt")]
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    USB_LP_CAN1_RX0 => embassy_stm32::usb::InterruptHandler<peripherals::USB>;
});

mod logic;
mod usb;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let mut config = Config::default();
    {
        use embassy_stm32::rcc::*;
        config.rcc.hse = Some(Hse {
            freq: Hertz(8_000_000),
            // Oscillator for bluepill, Bypass for nucleos.
            mode: HseMode::Oscillator,
        });
        config.rcc.pll = Some(Pll {
            src: PllSource::HSE,
            prediv: PllPreDiv::DIV1,
            mul: PllMul::MUL9,
        });
        config.rcc.sys = Sysclk::PLL1_P;
        config.rcc.ahb_pre = AHBPrescaler::DIV1;
        config.rcc.apb1_pre = APBPrescaler::DIV2;
        config.rcc.apb2_pre = APBPrescaler::DIV1;
    }

    let mut p = embassy_stm32::init(config);

    info!("Hello World!");

    {
        // BluePill board has a pull-up resistor on the D+ line.
        // Pull the D+ pin down to send a RESET condition to the USB bus.
        // This forced reset is needed only for development, without it host
        // will not reset your device when you upload new firmware.
        let _dp = Output::new(&mut p.PA12, Level::Low, Speed::Low);
        Timer::after_millis(10).await;
    }

    let usb_driver = embassy_stm32::usb::Driver::new(p.USB, Irqs, p.PA12, p.PA11);

    let mut usb_config = embassy_usb::Config::new(0xffff, 0xffff);
    usb_config.manufacturer = Some("3nt3");
    usb_config.product = Some("STM32 USB Blinkenlights");

    let mut config_descriptor = [0; 256];
    let mut bos_descriptor = [0; 256];
    let mut control_buf = [0; 256];

    let mut state = State::new();

    let mut builder = Builder::new(
        usb_driver,
        usb_config,
        &mut config_descriptor,
        &mut bos_descriptor,
        &mut [],
        &mut control_buf,
    );

    let mut class = CdcAcmClass::new(&mut builder, &mut state, 64);

    let mut usb = builder.build();

    let usb_fut = usb.run();

    let mut leds = [
        OutputOpenDrain::new(&mut p.PA0, Level::Low, Speed::Low),
        OutputOpenDrain::new(&mut p.PA1, Level::Low, Speed::Low),
        OutputOpenDrain::new(&mut p.PA2, Level::Low, Speed::Low),
        OutputOpenDrain::new(&mut p.PA3, Level::Low, Speed::Low),
        OutputOpenDrain::new(&mut p.PA4, Level::Low, Speed::Low),
        OutputOpenDrain::new(&mut p.PA5, Level::Low, Speed::Low),
        OutputOpenDrain::new(&mut p.PA6, Level::Low, Speed::Low),
        OutputOpenDrain::new(&mut p.PA7, Level::Low, Speed::Low),
        OutputOpenDrain::new(&mut p.PA8, Level::Low, Speed::Low),
        OutputOpenDrain::new(&mut p.PA9, Level::Low, Speed::Low),
    ];

    let usb_logic_fut = usb::usb_task(&mut class, &mut leds);

    join(usb_fut, usb_logic_fut).await;
}

struct Disconnected {}

impl From<EndpointError> for Disconnected {
    fn from(val: EndpointError) -> Self {
        match val {
            EndpointError::BufferOverflow => panic!("Buffer overflow"),
            EndpointError::Disabled => Disconnected {},
        }
    }
}
