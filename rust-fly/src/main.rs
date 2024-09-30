#![no_std]
#![no_main]
#![allow(async_fn_in_trait)]

mod pio_servo;

use core::f32::consts::PI;
use core::ops::{Add, Div};
use cortex_m::prelude::{_embedded_hal_blocking_i2c_Write, _embedded_hal_blocking_i2c_WriteRead};
use cyw43_pio::PioSpi;
use defmt::*;
use embassy_executor::Spawner;
use embassy_net::tcp::{Error, TcpSocket};
use embassy_net::{Config, StackResources};
use embassy_rp::bind_interrupts;
use embassy_rp::clocks::RoscRng;
use embassy_rp::gpio::{Level, Output};
use embassy_rp::i2c::{Async, I2c};
use embassy_rp::peripherals::{DMA_CH0, I2C0, I2C1, PIO0};
use embassy_rp::pio::{InterruptHandler, Pio};
use embassy_rp::pwm::Pwm;
use embassy_rp::rom_data::double_funcs::{datan2, ddiv};
use embassy_rp::rom_data::float_funcs::{fatan2, fdiv};
use embassy_time::Duration;
use embedded_io_async::Write;
use fixed::traits::ToFixed;
use rand::RngCore;
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};
use crate::pio_servo::{PwmPio, ServoBuilder};

#[embassy_executor::task]
async fn cyw43_task(runner: cyw43::Runner<'static, Output<'static>, PioSpi<'static, PIO0, 0, DMA_CH0>>) -> ! {
    runner.run().await
}

#[embassy_executor::task]
async fn net_task(mut runner: embassy_net::Runner<'static, cyw43::NetDriver<'static>>) -> ! {
    runner.run().await
}


#[cfg(debug_assertions)]
fn get_firmware() -> &'static [u8] {
    unsafe { core::slice::from_raw_parts(0x10100000 as *const u8, 230321) }
}
#[cfg(debug_assertions)]
fn get_firmware_clm() -> &'static [u8] {
    unsafe { core::slice::from_raw_parts(0x10140000 as *const u8, 4752) }
}

#[cfg(not(debug_assertions))]
fn get_firmware() -> &'static [u8] {
    include_bytes!("../src/lib/cyw43-firmware/43439A0.bin")
}
#[cfg(not(debug_assertions))]
fn get_firmware_clm() -> &'static [u8] {
    include_bytes!("../src/lib/cyw43-firmware/43439A0_clm.bin")
}
bind_interrupts!(struct Pio0Irqs {
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
});


#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());



    /*let Pio { mut common, sm0, .. } = Pio::new(p.PIO0, Pio0Irqs);
    let pwm_fly = PwmPio::new(&mut common, sm0, p.PIN_16);
    let mut servo = ServoBuilder::new(pwm_fly)
        // 设定运动范围
        .set_max_degree_rotation(100)
        // 测量出来的两个脉冲范围
        .set_min_pulse_width(core::time::Duration::from_micros(700))
        .set_max_pulse_width(core::time::Duration::from_micros(2000))
        .build();
    servo.start();

    let mut rotate = 0;
    let mut tp = true;
    loop {
        if tp {
            rotate = rotate + 5;
        } else {
            rotate = rotate - 5;
        }
        if rotate >= 100 {
            rotate = 100;
            tp = false;
        } else if rotate <= 0 {
            rotate = 0;
            tp = true;
        }
        info!("rotate >>{}", rotate);
        servo.rotate(rotate);
        embassy_time::Timer::after(embassy_time::Duration::from_millis(200)).await;
    }*/

    /*let mut rng = RoscRng;

    let fw = get_firmware();
    let clm = get_firmware_clm();

    let pwr = Output::new(p.PIN_23, Level::Low);
    let cs = Output::new(p.PIN_25, Level::High);
    let mut pio = Pio::new(p.PIO0, Irqs);
    let spi = PioSpi::new(&mut pio.common, pio.sm0, pio.irq0, cs, p.PIN_24, p.PIN_29, p.DMA_CH0);

    static STATE: StaticCell<cyw43::State> = StaticCell::new();
    let state = STATE.init(cyw43::State::new());
    let (net_device, mut control, runner) = cyw43::new(state, pwr, spi, fw).await;
    unwrap!(spawner.spawn(cyw43_task(runner)));

    control.init(clm).await;
    control
        .set_power_management(cyw43::PowerManagementMode::PowerSave)
        .await;

    // Use a link-local address for communication without DHCP server
    let config = Config::ipv4_static(embassy_net::StaticConfigV4 {
        address: embassy_net::Ipv4Cidr::new(embassy_net::Ipv4Address::new(169, 254, 1, 1), 16),
        dns_servers: heapless::Vec::new(),
        gateway: None,
    });
    let seed = rng.next_u64();
    static RESOURCES: StaticCell<StackResources<3>> = StaticCell::new();
    let (stack, runner) = embassy_net::new(net_device, config, RESOURCES.init(StackResources::new()), seed);

    unwrap!(spawner.spawn(net_task(runner)));

    control.start_ap_open("PICO_FLY", 5).await;
    // control.start_ap_wpa2("cyw43", "password", 5).await;

    let mut rx_buffer = [0; 4096];
    let mut tx_buffer = [0; 4096];
    let mut buf = [0; 4096];
    loop {
        let mut socket = TcpSocket::new(stack, &mut rx_buffer, &mut tx_buffer);
        socket.set_timeout(Some(Duration::from_secs(10)));

        control.gpio_set(0, false).await;
        info!("Listening on TCP:1234...");
        if let Err(e) = socket.accept(1234).await {
            warn!("accept error: {:?}", e);
            continue;
        }

        info!("Received connection from {:?}", socket.remote_endpoint());
        control.gpio_set(0, true).await;

        loop {
            let n = match socket.read(&mut buf).await {
                Ok(0) => { break; }
                Ok(n) => n,
                _ => { break; }
            };

            // 对收到的数据进行处理
            let data = &buf[..n];
        }
    }*/
}

fn heading_calc(x: f32, y: f32, z: f32) -> f32 {
    // https://www.magnetic-declination.com/
    // let declination_angle = (7.0 + (46.0 / 60.0)) / (180.0 / PI);
    let declination_angle = 0.0;
    let mut heading = fatan2(y, x) + declination_angle;
    if heading < 0.0 {
        heading += 2.0 * PI;
    }
    if heading > 2.0 * PI {
        heading -= 2.0 * PI;
    }
    heading * 180.0 / PI
}

struct HeadingOffset {
    baseline: f32,
}

impl HeadingOffset {
    // 初始化基准航向
    pub fn new(baseline: f32) -> Self {
        HeadingOffset { baseline }
    }

    // 计算偏移后的航向
    pub fn calculate_offset(&self, heading: f32) -> f32 {
        // 计算偏移量
        let offset = heading - self.baseline;

        // 将结果范围限制在0到360度之间
        (offset + 360.0) % 360.0
    }
}
