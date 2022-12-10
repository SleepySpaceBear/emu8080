union RegisterPair {
    as_u16: u16,
    as_u8: [u8; 2],
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
    status: u8
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
            status: 0x2
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
        ((self.b as u16) << 8) + self.c as u16
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
        ((self.d as u16) << 8) + self.e as u16
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
        ((self.h as u16) << 8) + self.l as u16
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
        ((self.accumulator as u16) << 8) + self.status as u16
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

    pub fn status_carry(&self) -> bool {
        return (self.status & 0x1) != 0; 
    }

    pub fn set_status_carry(&self, carry: bool) {
        if carry {
            self.status |= 0x1;
        }
        else {
            self.status &= !0x1;
        }
    }
    
    pub fn status_aux_carry(&self) -> bool {
        return (self.status & 0x10) != 0; 
    }

    pub fn set_status_aux_carry(&self, aux_carry: bool) {
        if aux_carry {
            self.status |= 0x10;
        }
        else {
            self.status &= !0x10;
        }
    }

    pub fn status_zero(&self) -> bool {
        return (self.status & 0x40) != 0; 
    }

    pub fn set_status_zero(&self, zero: bool) {
        if zero {
            self.status |= 0x40;
        }
        else {
            self.status &= !0x40;
        }
    }
    
    pub fn status_parity(&self) -> bool {
        return (self.status & 0x04) != 0; 
    }

    pub fn set_status_parity(&self, parity: bool) {
        if parity {
            self.status |= 0x04;
        }
        else {
            self.status &= !0x04;
        }
    }
    
    pub fn status_sign(&self) -> bool {
        return (self.status & 0x80) != 0; 
    }

    pub fn set_status_sign(&self, sign: bool) {
        if sign {
            self.status |= 0x80;
        }
        else {
            self.status &= !0x80;
        }
    }


}
