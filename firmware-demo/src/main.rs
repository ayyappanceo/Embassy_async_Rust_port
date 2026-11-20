#![no_std]
#![no_main]

use core::fmt::Write;
use core::mem::MaybeUninit;
use embassy_executor::{Executor, Spawner};
use embassy_time::Timer;
use embedded_hal::digital::OutputPin;
use embedded_hal::i2c::I2c;
use embedded_hal::spi::SpiBus;
use panic_halt as _;
use portable_atomic as _;
use riscv_rt::entry;
use thejas32_hal::{init_embassy_timer, Gpio0Pins, I2cMaster0, Serial0, SpiMaster0};
use thejas32_pac::Peripherals;

#[embassy_executor::task]
async fn led_heartbeat_task(mut led: thejas32_hal::Pin<13>) {
    for _ in 0..3 {
        led.set_high().unwrap();
        Timer::after_millis(20).await;
        led.set_low().unwrap();
        Timer::after_millis(20).await;
    }
}

#[embassy_executor::task]
async fn main_task(spawner: Spawner) {
    let p = unsafe { Peripherals::steal() };
    init_embassy_timer();

    let mut serial = Serial0::new(p.uart0);
    let pins = Gpio0Pins::split(p.gpio0);
    let led = pins.p13.into_output();
    let mut spi = SpiMaster0::new(p.spi0, 8);
    let mut i2c = I2cMaster0::new(p.i2c0, 100);

    writeln!(serial, "=== THEJAS32 Embassy Async + SPI/I2C HAL ===").unwrap();
    spawner.spawn(led_heartbeat_task(led)).unwrap();

    let mut rx = [0u8; 2];
    spi.transfer(&mut rx, &[0xAA, 0x55]).unwrap();
    writeln!(serial, "[SPI0] TX=[0xAA, 0x55] -> RX=[0x{:02X}, 0x{:02X}]", rx[0], rx[1]).unwrap();

    let mut who_am_i = [0u8; 1];
    i2c.write_read(0x68, &[0x75], &mut who_am_i).unwrap();
    writeln!(serial, "[I2C0] MPU6050 WHO_AM_I = 0x{:02X}", who_am_i[0]).unwrap();

    i2c.write(0x3C, &[0x00, 0xAE, 0xAF]).unwrap();
    writeln!(serial, "[I2C0] SSD1306 Display ON packet sent").unwrap();

    Timer::after_millis(150).await;
    writeln!(serial, "=== All Async Tasks & Drivers Verified ===").unwrap();
    loop { Timer::after_secs(1).await; }
}

static mut EXECUTOR: MaybeUninit<Executor> = MaybeUninit::uninit();

#[entry]
fn main() -> ! {
    let executor = unsafe {
        let ptr = core::ptr::addr_of_mut!(EXECUTOR);
        (*ptr).write(Executor::new());
        (*ptr).assume_init_mut()
    };
    executor.run(|spawner| {
        spawner.spawn(main_task(spawner)).unwrap();
    });
}
