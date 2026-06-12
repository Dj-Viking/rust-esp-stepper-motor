#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![allow(unused)]

use embassy_executor::Spawner;
use embassy_time::{Timer};
use esp_hal::clock::CpuClock;
use esp_hal::delay::Delay;
use esp_hal::time::Rate;
use esp_hal::rmt::Rmt;
use esp_hal::gpio::{Output, OutputConfig};
use esp_hal::timer::systimer::SystemTimer;
use esp_hal::rmt::{PulseCode, TxChannelCreator, TxChannelConfig, TxChannelAsync};
use esp_hal::gpio::Level;
use esp_hal::usb_serial_jtag::UsbSerialJtag;
use core::fmt::Write;
use embedded_io_async::Read;

fn blink_once(led: &mut Output<'_>, delay: &mut Delay) {
    led.set_high();
    delay.delay_millis(10);
    led.set_low();
    delay.delay_millis(10);
}

#[panic_handler]
fn panic(p: &core::panic::PanicInfo) -> ! {
    let mut usb = UsbSerialJtag::new(unsafe { esp_hal::peripherals::USB_DEVICE::steal() });

    if let Some(l) = p.location() {
        let _ = write!(usb, "{}: ", l);
    }

    let _ = write!(usb, "{}", p.message());

    loop {}
}
// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

const LED_COUNT: usize = 2;

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {

	// let mut delay = Delay::new();

    // let peripherals = esp_hal::init(
    //     esp_hal::Config::default()
    //         .with_cpu_clock(CpuClock::max()));

    // // esp_hal_embassy::init(SystemTimer::new(peripherals.SYSTIMER).alarm0);

    // // let mut usb_async = UsbSerialJtag::new(peripherals.USB_DEVICE).into_async();

	// let _enable = Output::new(peripherals.GPIO5,
    //     		esp_hal::gpio::Level::High,
	// 			OutputConfig::default());

	// let mut m0   = peripherals.GPIO6;
	// let mut m1   = peripherals.GPIO7;
	// let mut m2   = peripherals.GPIO8;

	// let mut rst  = peripherals.GPIO9;
	// let mut step = peripherals.GPIO0;
	// let mut dir  = peripherals.GPIO1;
	// let mut slp  = peripherals.GPIO4;

	// let op0        = Output::new(m0,   Level::High, OutputConfig::default());
	// let op1        = Output::new(m1,   Level::Low,  OutputConfig::default());
	// let op2        = Output::new(m2,   Level::Low,  OutputConfig::default());

	// let oprst      = Output::new(rst,  Level::Low,  OutputConfig::default());
	// let mut opstep = Output::new(step, Level::High, OutputConfig::default());
	// let opdir      = Output::new(dir,  Level::Low,  OutputConfig::default());
	// let opslp      = Output::new(slp,  Level::Low,  OutputConfig::default());

	// data is a sequence of 24bit GRB colors, as [u8; _]
	// start off with some green
	let mut data: [u8; LED_COUNT*3] = [
	//   G  R  B
		20, 0, 0, // led 0
		0,  0, 20  // led 1
		// ... 4096
	];

	// let mut usb_buf: [u8; LED_COUNT*3] = [0; _];

	// spawner.spawn(task()).unwrap();

	loop {
		// if swapping to reading from this usb to not read from usb
		// then you have to disconnect IO9 and then reset it after plugging it in
		// to power again

		// usb_async.read_exact(&mut usb_buf).await.unwrap();

		// let mut pulses: [u32; LED_COUNT*24 + 1] = [0; _];

		// data.map(byte_to_pulses).iter().enumerate().for_each(|(i, e)|
		// 	pulses[i * 8..(i + 1) * 8].copy_from_slice(e));
		// usb_buf.map(byte_to_pulses).iter().enumerate().for_each(|(i, e)|
		// 	pulses[i * 8..(i + 1) * 8].copy_from_slice(e));

		// pulses[pulses.len()-1] = PulseCode::new(Level::Low, 0, Level::Low, 0);
		// channel.transmit(&pulses).await.unwrap();

		// task() in the background
		//

		// Timer::after_millis(20).await;

		// blink_once(&mut opstep, &mut delay);


		// fades from 0 blue to max blue until it wraps around to 0 again
		// data[2] = data[2].wrapping_add(10);
	}
}


fn byte_to_pulses(b: u8) -> [u32; 8] {
    [7, 6, 5, 4, 3, 2, 1, 0].map(|i| match (b >> i) & 1 {
        0 => PulseCode::new(Level::High, 32, Level::Low, 68),
        _ => PulseCode::new(Level::High, 64, Level::Low, 36),
    })
}