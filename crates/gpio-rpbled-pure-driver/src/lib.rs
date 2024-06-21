#![no_std]
use core::ptr::NonNull;

use tock_registers::interfaces::{Readable, Writeable};
use tock_registers::register_structs;
use tock_registers::registers::{ReadOnly, ReadWrite, WriteOnly};


register_structs! {
    #[allow(non_snake_case)]
    pub GpioRegisters {
        (0x00 => GPFSEL: [ReadWrite<u32>; 6]),
        (0x18 => _pad0),
        (0x1c => GPSET: [ReadWrite<u32>; 2]),
        (0x24 => _pad1),
        (0x28 => GPCLR: [WriteOnly<u32>; 2]),
        (0x30 => _pad2),
        (0x34 => GPLEV: [ReadOnly<u32>; 2]),
        (0x3c => _pad3),
        (0x40 => GPEDS: [ReadWrite<u32>; 2]),
        (0x48 => _pad4),
        (0x4c => GPREN: [ReadWrite<u32>; 2]),
        (0x54 => _pad5),
        (0x58 => GPFEN: [ReadWrite<u32>; 2]),
        (0x60 => _pad6),
        (0x64 => GPHEN: [ReadWrite<u32>; 2]),
        (0x6c => _pad7),
        (0x70 => GPLEN: [ReadWrite<u32>; 2]),
        (0x78 => _pad8),
        (0x7c => GPAREN: [ReadWrite<u32>; 2]),
        (0x84 => _pad9),
        (0x88 => GPAFEN: [ReadWrite<u32>; 2]),
        (0x90 => _pad10),
        (0xe4 => GPIO_PUP_PDN_CNTRL_REG: [ReadWrite<u32>; 4]),
        (0xf4 => @END),
    }
}

pub struct GpioPort {
    pub regs: NonNull<GpioRegisters>
}

impl GpioPort {
    pub fn new(base_addr: *mut u8) -> Self {
        Self { regs: NonNull::new(base_addr).unwrap().cast() }
    }

    fn regs(&self) -> &GpioRegisters {
        unsafe {
            self.regs.as_ref()
        }
    }

    pub fn set_as_output(&self, pin_id: usize) {
        let idx = pin_id / 10;
        let data = self.regs().GPFSEL[idx].get();
        self.regs().GPFSEL[idx].set(data & (!(0b111u32 << (3 * (pin_id % 10)))));
        self.regs().GPFSEL[idx].set(data | (1u32 << (3 * (pin_id % 10)))); 
    }

    pub fn get_state(&self, pin_id: usize) -> u32 {
        let idx = pin_id / 32;
        self.regs().GPLEV[idx].get() >> (pin_id % 32) & 1
    }

    pub fn led_on(&self, pin_id: usize) {
        let (group, offset) = (pin_id / 32, pin_id % 32);
        self.regs().GPSET[group].set(1 << offset);
    }

    pub fn led_off(&self, pin_id: usize) {
        let (group, offset) = (pin_id / 32, pin_id % 32);
        self.regs().GPCLR[group].set(1 << offset);
    }
}