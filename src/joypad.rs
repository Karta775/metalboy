use crate::{check_bit, unset_bit};
use crate::mmu::Mmu;

pub const JOYP: u16 = 0xFF00;

pub enum Button {
    Up,
    Down,
    Left,
    Right,
    A, B,
    Start,
    Select,
}

pub struct Joypad {}

impl Joypad {
    pub fn update(mmu: &mut Mmu, pressed: &[Button]) {
        let mut control = mmu.get(JOYP);
        control |= 0x0f; // Clear the previous button presses

        if !check_bit(control, 4) { // Direction mode
            for button in pressed.iter() {
                match button {
                    Button::Right => unset_bit(&mut control, 0),
                    Button::Left => unset_bit(&mut control, 1),
                    Button::Up => unset_bit(&mut control, 2),
                    Button::Down => unset_bit(&mut control, 3),
                    _ => {}
                }
            }
        }
        if !check_bit(control, 5) { // Action mode
            for button in pressed.iter() {
                match button {
                    Button::A => unset_bit(&mut control, 0),
                    Button::B => unset_bit(&mut control, 1),
                    Button::Select => unset_bit(&mut control, 2),
                    Button::Start => unset_bit(&mut control, 3),
                    _ => {}
                }
            }
        }

        mmu.set_joypad_buttons(control);
        if !pressed.is_empty() {
            mmu.request_interrupt(4);
        }
    }
}