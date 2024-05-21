#![no_main]
#![no_std]
#![feature(macro_metavar_expr)]
#![feature(type_alias_impl_trait)]
#![feature(generic_const_exprs)]

use defmt_rtt as _;
use panic_probe as _;
use rumcake::keyberon;
use rumcake::keyboard;
use rumcake::keyboard::{build_layout, build_analog_matrix, KeyboardMatrix, KeyboardLayout, Keyboard};
use rumcake::hw::platform::setup_adc_sampler;
#[keyboard(
    usb,
    bluetooth
)]
pub struct Macropad;

// Basic keyboard configuration
impl Keyboard for Macropad {
    const MANUFACTURER: &'static str = "Proctor Technologies";
    const PRODUCT: &'static str = "Macropad";
    const SERIAL_NUMBER: &'static str = "deadbeef";

}

setup_adc_sampler! {
    (timer: TIMER2, ppi_ch0: PPI_CH2, ppi_ch1: PPI_CH3) => {
        Multiplexer {
            pin: P0_31, // MCU analog pin connected to a multiplexer
            select_pins: [ P0_22 P0_24 P1_00 P0_11 ] // Pins connected to the selection pins on the multiplexer
        }
    }
}


impl KeyboardMatrix for Macropad {
    type Layout = Self;

    build_analog_matrix! {
        channels: {
            [(0, 0) (0, 1) (0, 2) (0, 3)]
            [(1, 0) (1, 1) (1, 2) (1, 3)]
            [(2, 3) (2, 2) (2, 1) (2, 0)]
            [(3, 3) (3, 2) (3, 1) (3, 0)]
        },
        ranges: {
            [0..1024 0..1024 0..1024 0..1024]
            [0..1024 0..1024 0..1024 0..1024]
            [0..1024 0..1024 0..1024 0..1024]
            [0..1024 0..1024 0..1024 0..1024]
        }
    }
}

impl KeyboardLayout for Macropad {
    // Use the remapping above to create the keyboard layout
        build_layout! {
            {
                [1 2 3 4]
                [Q W E R]
                [A S D F]
                [Z X C V]
            }
        }
}

// USB configuration
use rumcake::usb::USBKeyboard;
impl USBKeyboard for Macropad  {
    const USB_VID: u16 = 0x0000; // TODO: Change this
    const USB_PID: u16 = 0x0000; // TODO: Change this
}

use rumcake::hw::platform::BluetoothDevice;

impl BluetoothDevice for Macropad {
    const BLUETOOTH_ADDRESS: [u8; 6] = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00]; // TODO: Change this
}

use rumcake::bluetooth::BluetoothKeyboard;
impl BluetoothKeyboard for Macropad {
    const BLE_VID: u16 = 0x0000; // TODO: Change this
    const BLE_PID: u16 = 0x0000; // TODO: Change this
}