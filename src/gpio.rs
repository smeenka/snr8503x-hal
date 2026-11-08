use snr8503_pac::{gpio0, exti};

use core::default::Default;
use core::derive;
use core::clone::Clone;
use core::cmp::PartialEq;

#[derive(Clone, PartialEq)]
pub enum Port {
    Port0,
    Port1,
}

#[derive(Clone, PartialEq)]
pub enum Mode {
    Input = 0,
    Output = 1,
    Analog = 2,
}

#[derive(Clone, PartialEq)]
pub enum PuPd {
    NoPull = 0,
    Up = 1,
    Down = 2,
}

#[derive(Clone, PartialEq)]
pub enum BitAction {
    Reset = 0,
    Set = 1,
}

#[derive(Clone, PartialEq, )]
pub enum ExtiTrigger {
    None = 0x00,
    NegEdge = 0x01,
    PosEdge = 0x02,
    BothEdges = 0x03,
}

#[derive(Clone)]
pub struct InitStruct {
    pub pin: u16, // Bitmask of pins
    pub mode: Mode,
    pub pupd: PuPd,
    pub pode_enable: bool,
}

impl Default for InitStruct {
    fn default() -> Self {
        Self {
            pin: 0,
            mode: Mode::Input,
            pupd: PuPd::NoPull,
            pode_enable: false,
        }
    }
}

pub fn init(port: &gpio0::RegisterBlock, config: &InitStruct) {
    let mut pinpos: u32;
    let mut pos: u32;
    let mut currentpin: u32;

    for i in 0..16 {
        pinpos = i;
        pos = 1 << pinpos;
        currentpin = (config.pin as u32) & pos;

        if currentpin == pos {
            // Mode Configuration
            match config.mode {
                Mode::Input => {
                    // Enable Input (PIE |= BIT)
                    port.gpio_pie().modify(|r, w| unsafe { w.bits(r.bits() | pos) });
                    // Disable Output (POE &= ~BIT)
                    port.gpio_poe().modify(|r, w| unsafe { w.bits(r.bits() & !pos) });
                }
                Mode::Output => {
                    // Disable Input (PIE &= ~BIT)
                    port.gpio_pie().modify(|r, w| unsafe { w.bits(r.bits() & !pos) });
                    // Enable Output (POE |= BIT)
                    port.gpio_poe().modify(|r, w| unsafe { w.bits(r.bits() | pos) });
                }
                Mode::Analog => {
                    // Disable Output (POE &= ~BIT)
                    port.gpio_poe().modify(|r, w| unsafe { w.bits(r.bits() & !pos) });
                    // Note: C code doesn't explicitly disable Input for Analog, but usually you would.
                    // The C code for Analog only touches POE.
                }
            }

            // Pull Up/Down Configuration
            match config.pupd {
                PuPd::Up => {
                    port.gpio_pue().modify(|r, w| unsafe { w.bits(r.bits() | pos) });
                }
                PuPd::Down => {
                    // C code does nothing for Down
                }
                PuPd::NoPull => {
                    port.gpio_pue().modify(|r, w| unsafe { w.bits(r.bits() & !pos) });
                }
            }

            // Open Drain Configuration
            if config.pode_enable {
                port.gpio_pode().modify(|r, w| unsafe { w.bits(r.bits() | pos) });
            } else {
                port.gpio_pode().modify(|r, w| unsafe { w.bits(r.bits() & !pos) });
            }
        }
    }
}

pub fn read_input_data_bit(port: &gpio0::RegisterBlock, pin: u16) -> bool {
    (port.gpio_pdi().read().bits() & (pin as u32)) != 0
}

pub fn read_input_data(port: &gpio0::RegisterBlock) -> u32 {
    port.gpio_pdi().read().bits()
}

pub fn read_output_data_bit(port: &gpio0::RegisterBlock, pin: u16) -> bool {
    (port.gpio_pdo().read().bits() & (pin as u32)) != 0
}

pub fn read_output_data(port: &gpio0::RegisterBlock) -> u32 {
    port.gpio_pdo().read().bits()
}

pub fn set_bits(port: &gpio0::RegisterBlock, pin: u16) {
    port.gpio_bsrr().write(|w| unsafe { w.bits(pin as u32) });
}

pub fn reset_bits(port: &gpio0::RegisterBlock, pin: u16) {
    port.gpio_brr().write(|w| unsafe { w.bits(pin as u32) });
}

pub fn write_bit(port: &gpio0::RegisterBlock, pin: u16, val: BitAction) {
    match val {
        BitAction::Set => set_bits(port, pin),
        BitAction::Reset => reset_bits(port, pin),
    }
}

pub fn write(port: &gpio0::RegisterBlock, val: u32) {
    port.gpio_pdo().write(|w| unsafe { w.bits(val) });
}

pub fn pin_af_config(port: &gpio0::RegisterBlock, pin_source: u32, af: u32) {
    // pin_source is 0..15
    // af is the Alternate Function number
    
    // Each register holds 4 pins (4 bits each) -> 16 bits total used?
    // C code:
    // F3210: pins 0-3
    // F7654: pins 4-7
    // FBA98: pins 8-11
    // FFEDC: pins 12-15
    
    // The registers in C code seem to be 16-bit or accessed as such if masks are 0xFFF0 etc?
    // In PAC, registers are u32 usually. Let's check bits() usage.
    // The C code uses `& 0xFFF0` which implies working on the lower 16 bits or similar. 
    // Wait, 4 pins * 4 bits = 16 bits. Yes.
    
    let shift = (pin_source % 4) * 4;
    let mask = !(0xF << shift);
    let af_val = af << shift;

    if pin_source < 4 {
        port.gpio_f3210().modify(|r, w| unsafe { w.bits((r.bits() & mask) | af_val) });
    } else if pin_source < 8 {
        port.gpio_f7654().modify(|r, w| unsafe { w.bits((r.bits() & mask) | af_val) });
    } else if pin_source < 12 {
        port.gpio_fba98().modify(|r, w| unsafe { w.bits((r.bits() & mask) | af_val) });
    } else if pin_source < 16 {
        port.gpio_ffedc().modify(|r, w| unsafe { w.bits((r.bits() & mask) | af_val) });
    }
}

pub fn exti_trigger_config(
    port_idx: Port, 
    pin_source: u32, 
    trigger: ExtiTrigger, 
    exti: &exti::RegisterBlock
) {
    let t_val = trigger as u32;
    
    match port_idx {
        Port::Port0 => {
            match pin_source {
                0 => { exti.exti_cr0().modify(|r, w| unsafe { w.bits((r.bits() & 0xfffc) | t_val) }); },
                2 => { exti.exti_cr0().modify(|r, w| unsafe { w.bits((r.bits() & 0xfff3) | (t_val << 2)) }); },
                4 => { exti.exti_cr0().modify(|r, w| unsafe { w.bits((r.bits() & 0xffcf) | (t_val << 4)) }); },
                5 => { exti.exti_cr0().modify(|r, w| unsafe { w.bits((r.bits() & 0xff3f) | (t_val << 6)) }); },
                6 => { exti.exti_cr0().modify(|r, w| unsafe { w.bits((r.bits() & 0xfcff) | (t_val << 8)) }); },
                7 => { exti.exti_cr0().modify(|r, w| unsafe { w.bits((r.bits() & 0xf3ff) | (t_val << 10)) }); },
                8 => { exti.exti_cr0().modify(|r, w| unsafe { w.bits((r.bits() & 0xcfff) | (t_val << 12)) }); },
                9 => { exti.exti_cr0().modify(|r, w| unsafe { w.bits((r.bits() & 0x3fff) | (t_val << 14)) }); },
                14 => { exti.exti_cr1().modify(|r, w| unsafe { w.bits((r.bits() & 0xfffc) | t_val) }); },
                15 => { exti.exti_cr1().modify(|r, w| unsafe { w.bits((r.bits() & 0xfff3) | (t_val << 2)) }); },
                _ => {}
            }
        }
        Port::Port1 => {
            match pin_source {
                4 => { exti.exti_cr1().modify(|r, w| unsafe { w.bits((r.bits() & 0xffcf) | (t_val << 4)) }); },
                5 => { exti.exti_cr1().modify(|r, w| unsafe { w.bits((r.bits() & 0xff3f) | (t_val << 6)) }); },
                6 => { exti.exti_cr1().modify(|r, w| unsafe { w.bits((r.bits() & 0xfcff) | (t_val << 8)) }); },
                7 => { exti.exti_cr1().modify(|r, w| unsafe { w.bits((r.bits() & 0xf3ff) | (t_val << 10)) }); },
                8 => { exti.exti_cr1().modify(|r, w| unsafe { w.bits((r.bits() & 0xcfff) | (t_val << 12)) }); },
                9 => { exti.exti_cr1().modify(|r, w| unsafe { w.bits((r.bits() & 0x3fff) | (t_val << 14)) }); },
                _ => {}
            }
        }
    }
}
