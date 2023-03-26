use crate::utils::make_u16;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
enum StatusFlags {
    SignBit = 0x80,
    ZeroBit = 0x40,
    AuxCarryBit = 0x10,
    ParityBit = 0x04,
    CarryBit = 0x01
}

pub struct Registers {
    pc: u16, // Program Counter
    sp: u16, // Stack Pointer
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    accumulator: u8,
    status: u8,
    w: u8,
    z: u8
}

impl Registers {
    pub fn new() -> Self {
        Self {
            pc: 0,
            sp: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            accumulator: 0,
            status: 0x2,
            w: 0,
            z: 0
        }
    }

    pub fn pc(&self) -> u16 {
        self.pc
    }

    pub fn set_pc(&mut self, val: u16) {
        self.pc = val
    }


    pub fn sp(&self) -> u16 {
        self.sp
    }

    pub fn set_sp(&mut self, val: u16) {
        self.sp = val
    }


    pub fn pair_b(&self) -> u16 {
        make_u16(self.b, self.c)
    }

    pub fn set_pair_b(&mut self, val: u16) {
       self.b = ((val >> 8) & 0xFF) as u8;
       self.c = (val & 0xFF) as u8
    }

    pub fn b(&self) -> u8 {
        self.b
    }

    pub fn set_b(&mut self, val: u8) {
        self.b = val 
    }

    pub fn c(&self) -> u8 {
        self.c 
    }

    pub fn set_c(&mut self, val: u8) {
        self.c = val 
    }


    pub fn pair_d(&self) -> u16 {
        make_u16(self.d, self.e)
    }

    pub fn set_pair_d(&mut self, val: u16) {
       self.d = ((val >> 8) & 0xFF) as u8;
       self.e = (val & 0xFF) as u8
    }

    pub fn d(&self) -> u8 {
        self.d 
    }

    pub fn set_d(&mut self, val: u8) {
        self.d = val 
    }

    pub fn e(&self) -> u8 {
        self.e 
    }

    pub fn set_e(&mut self, val: u8) {
        self.e = val 
    }


    pub fn pair_h(&self) -> u16 {
        make_u16(self.h, self.l)
    }

    pub fn set_pair_h(&mut self, val: u16) {
       self.h = ((val >> 8) & 0xFF) as u8;
       self.l = (val & 0xFF) as u8
    } 

    pub fn h(&self) -> u8 {
        self.h 
    }

    pub fn set_h(&mut self, val: u8) {
        self.h = val 
    }

    pub fn l(&self) -> u8 {
        self.l 
    }

    pub fn set_l(&mut self, val: u8) {
        self.l = val
    }


    pub fn psw(&self) -> u16 {
        make_u16(self.accumulator, self.status)
    }

    pub fn set_psw(&mut self, val: u16) {
       self.accumulator = ((val >> 8) & 0xFF) as u8;
       self.status = (val & 0xFF) as u8
    }

    pub fn accumulator(&self) -> u8 {
        self.accumulator 
    }

    pub fn set_accumulator(&mut self, val: u8) {
        self.accumulator = val
    }

    pub fn status(&self) -> u8 {
        self.status
    }

    pub fn set_status(&self, val: u8) {
        self.status = 0x2 | (val & 0xC5);
    }

    pub fn status_carry(&self) -> bool {
        return (self.status & (StatusFlags::CarryBit as u8)) != 0; 
    }

    pub fn set_status_carry(&mut self, carry: bool) {
        if carry {
            self.status |= StatusFlags::CarryBit as u8;
        }
        else {
            self.status &= !(StatusFlags::CarryBit as u8);
        }
    }
    
    pub fn status_aux_carry(&self) -> bool {
        return (self.status & (StatusFlags::AuxCarryBit as u8)) != 0; 
    }

    pub fn set_status_aux_carry(&mut self, aux_carry: bool) {
        if aux_carry {
            self.status |= StatusFlags::AuxCarryBit as u8;
        }
        else {
            self.status &= !(StatusFlags::AuxCarryBit as u8);
        }
    }

    pub fn status_zero(&self) -> bool {
        return (self.status & 0x40) != 0; 
    }

    pub fn set_status_zero(&mut self, zero: bool) {
        if zero {
            self.status |= StatusFlags::ZeroBit as u8;
        }
        else {
            self.status &= !(StatusFlags::ZeroBit as u8);
        }
    }
    
    pub fn status_parity(&self) -> bool {
        return (self.status & (StatusFlags::ParityBit as u8)) != 0; 
    }

    pub fn set_status_parity(&mut self, parity: bool) {
        if parity {
            self.status |= StatusFlags::ParityBit as u8;
        }
        else {
            self.status &= !(StatusFlags::ParityBit as u8);
        }
    }
    
    pub fn status_sign(&self) -> bool {
        return (self.status & (StatusFlags::SignBit as u8)) != 0; 
    }

    pub fn set_status_sign(&mut self, sign: bool) {
        if sign {
            self.status |= StatusFlags::SignBit as u8;
        }
        else {
            self.status &= !(StatusFlags::SignBit as u8);
        }
    }

    pub fn w(&self) -> u8 {
       self.w 
    }

    pub fn set_w(&mut self, val: u8) {
        self.w = val;
    }
    
    pub fn z(&self) -> u8 {
       self.z 
    }

    pub fn set_z(&mut self, val: u8) {
        self.z = val;
    }
}
