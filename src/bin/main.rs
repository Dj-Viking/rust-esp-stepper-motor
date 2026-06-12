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

// if the main function panics during monitoring `sh build.sh -m`
// (if monitoring works), the panic message will be sent to the console through the usb JTAG back
// to the computer terminal from the esp 
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

    let peripherals = esp_hal::init(
        esp_hal::Config::default()
            .with_cpu_clock(CpuClock::max()));

	// I don't remember what this is for :)
    // esp_hal_embassy::init(SystemTimer::new(peripherals.SYSTIMER).alarm0);

	// pins [according to that DRV8825 motor module - 
	// pins will have to change if we change the
	// motor module we wanna use
	let _enable = Output::new(peripherals.GPIO5,
        		esp_hal::gpio::Level::High,
				OutputConfig::default());

		// step degree subdivisions
	let mut m0    = peripherals.GPIO6;
	let mut m1    = peripherals.GPIO7;
	let mut m2    = peripherals.GPIO8;
		// step degree subdivisions

	let mut step  = peripherals.GPIO0;
	
	let mut reset = peripherals.GPIO9;
	let mut dir   = peripherals.GPIO1;
	let mut slp   = peripherals.GPIO4;
	// pins 

	// outputs
	let op0             = Output::new(m0,    Level::High, OutputConfig::default());
	let op1             = Output::new(m1,    Level::Low,  OutputConfig::default());
	let op2             = Output::new(m2,    Level::Low,  OutputConfig::default());

	let oprst           = Output::new(reset, Level::Low,  OutputConfig::default());
	let opdir           = Output::new(dir,   Level::Low,  OutputConfig::default());
	let opslp           = Output::new(slp,   Level::Low,  OutputConfig::default());

	let mut step_output = Output::new(step,  Level::High, OutputConfig::default());
	// outputs

	let mut delay       = Delay::new();


	loop {
		step_on_delay(&mut step_output, &mut delay);
	}
}

fn step_on_delay(step_output: &mut Output<'_>, delay: &mut Delay) {
    step_output.set_high();
    delay.delay_millis(10);
    step_output.set_low();
    delay.delay_millis(10);
}

fn byte_to_pulses(b: u8) -> [u32; 8] {
    [7, 6, 5, 4, 3, 2, 1, 0].map(|i| match (b >> i) & 1 {
        0 => PulseCode::new(Level::High, 32, Level::Low, 68),
        _ => PulseCode::new(Level::High, 64, Level::Low, 36),
    })
}
