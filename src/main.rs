#![no_main]
#![no_std]
#![feature(macro_metavar_expr)]
#![feature(type_alias_impl_trait)]
#![feature(generic_const_exprs)]

// TODO: Edit the `definition.json` file in the `src` folder to match your keyboard.
// _generated.rs is generated at build time, and will contain a serialized version of your Vial definition file to be compiled into your firmware.
// This file won't be generated if you don't specify the "vial" feature flag for rumcake.
#[cfg(vial)]
include!(concat!(env!("OUT_DIR"), "/_generated.rs"));

use defmt_rtt as _;
use panic_probe as _;
use rumcake::keyberon;
use rumcake::keyboard;
use rumcake::keyboard::build_layout;

#[keyboard(
    usb,
    bluetooth
)]
pub struct Macropad;

// Basic keyboard configuration
use rumcake::keyboard::Keyboard;
impl Keyboard for Macropad {
    const MANUFACTURER: &'static str = "Proctor Technologies";
    const PRODUCT: &'static str = "Macropad";
    const SERIAL_NUMBER: &'static str = "deadbeef";

}

// Layout configuration
use rumcake::keyboard::KeyboardLayout;
impl KeyboardLayout for Macropad {
    // Use the remapping above to create the keyboard layout
        build_layout! {
            {
                [1  2   3   4]
                [Q  W   E   R]
                [A  S   D   F]
                [Z  X   C   V]
            }
        }
}
// Matrix configuration
use rumcake::keyboard::{build_analog_matrix, KeyboardMatrix};
setup_adc_sampler! {
    
}
impl KeyboardMatrix for Macropad {
    type Layout = Self;
    build_analog_matrix!{
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

// USB configuration
use rumcake::usb::USBKeyboard;
impl USBKeyboard for Macropad  {
    const USB_VID: u16 = 0x0000; // TODO: Change this
    const USB_PID: u16 = 0x0000; // TODO: Change this
}

// Via configuration
// Note: since the `storage` feature flag is not enabled, changes to your keyboard in the Vial app will not be saved. If you use `storage`, be sure to update memory.x.
struct MacropadVia;
use rumcake::via::ViaKeyboard;
impl ViaKeyboard for MacropadVia {
    type Layout = Macropad;
}
use rumcake::vial::VialKeyboard;
impl VialKeyboard for MacropadVia {
    const VIAL_KEYBOARD_UID: [u8; 8] = [0xde, 0xad, 0xbe, 0xef, 0xfe, 0xeb, 0xda, 0xed];
    const VIAL_UNLOCK_COMBO: &'static [(u8, u8)] = &[(2, 4)];
    const KEYBOARD_DEFINITION: &'static [u8] = &GENERATED_KEYBOARD_DEFINITION;
}
