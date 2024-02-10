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
        self.set_status((val & 0xFF) as u8);
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

    pub fn set_status(&mut self, val: u8) {
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
        return (self.status & StatusFlags::ZeroBit as u8) != 0; 
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

    pub fn pair_w(&self) -> u16 {
       make_u16(self.w(), self.z())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_reg_pair_b() {
        let mut regs = Registers::new();

        regs.set_b(0x5);
        assert_eq!(regs.b(), 0x5);

        regs.set_c(0x7);
        assert_eq!(regs.c(), 0x7);

        assert_eq!(regs.pair_b(), 0x0507);

        regs.set_pair_b(0x0705);
        assert_eq!(regs.b(), 0x7);
        assert_eq!(regs.c(), 0x5);
    }

    #[test]
    fn test_reg_pair_d() {
        let mut regs = Registers::new();
        
        regs.set_d(0x5);
        assert_eq!(regs.d(), 0x5);

        regs.set_e(0x7);
        assert_eq!(regs.e(), 0x7);

        assert_eq!(regs.pair_d(), 0x0507);
        
        regs.set_pair_d(0x0705);
        assert_eq!(regs.d(), 0x7);
        assert_eq!(regs.e(), 0x5);
    }
   
    #[test]
    fn test_reg_pair_h() {
        let mut regs = Registers::new();
        
        regs.set_h(0x5);
        assert_eq!(regs.h(), 0x5);

        regs.set_l(0x7);
        assert_eq!(regs.l(), 0x7);

        assert_eq!(regs.pair_h(), 0x0507);

        regs.set_pair_h(0x0705);
        assert_eq!(regs.h(), 0x7);
        assert_eq!(regs.l(), 0x5);
    }

    #[test]
    fn test_reg_pair_w() {
        let mut regs = Registers::new();
        
        regs.set_w(0x5);
        assert_eq!(regs.w(), 0x5);

        regs.set_z(0x7);
        assert_eq!(regs.z(), 0x7);

        assert_eq!(regs.pair_w(), 0x0507);
    }

    #[test]
    fn test_status() {
        let mut regs = Registers::new();

        // verify that certain bits in the PSW can't have invalid values
        regs.set_psw(0xFFFF);
        assert_eq!(regs.accumulator(), 0xFF);
        assert_eq!(regs.status(), 0xC7);
        
        regs.set_psw(0x0000);
        assert_eq!(regs.accumulator(), 0x00);
        assert_eq!(regs.status(), 0x02);
        
        regs.set_status(0xFF);
        assert_eq!(regs.status(), 0xC7);

        regs.set_status(0x00);
        assert_eq!(regs.status(), 0x02);


        regs.set_status_sign(true);
        assert_ne!(regs.status() & (StatusFlags::SignBit as u8), 0);
        assert!(regs.status_sign());

        regs.set_status_sign(false);
        assert_eq!(regs.status() & (StatusFlags::SignBit as u8), 0);
        assert!(!regs.status_sign());


        regs.set_status_zero(true);
        assert_ne!(regs.status() & (StatusFlags::ZeroBit as u8), 0);
        assert!(regs.status_zero());

        regs.set_status_zero(false);
        assert_eq!(regs.status() & (StatusFlags::ZeroBit as u8), 0);
        assert!(!regs.status_zero());


        regs.set_status_aux_carry(true);
        assert_ne!(regs.status() & (StatusFlags::AuxCarryBit as u8), 0);
        assert!(regs.status_aux_carry());

        regs.set_status_aux_carry(false);
        assert_eq!(regs.status() & (StatusFlags::AuxCarryBit as u8), 0);
        assert!(!regs.status_aux_carry());


        regs.set_status_parity(true);
        assert_ne!(regs.status() & (StatusFlags::ParityBit as u8), 0);
        assert!(regs.status_parity());

        regs.set_status_parity(false);
        assert_eq!(regs.status() & (StatusFlags::ParityBit as u8), 0);
        assert!(!regs.status_parity());


        regs.set_status_carry(true);
        assert_ne!(regs.status() & (StatusFlags::CarryBit as u8), 0);
        assert!(regs.status_carry());

        regs.set_status_carry(false);
        assert_eq!(regs.status() & (StatusFlags::CarryBit as u8), 0);
        assert!(!regs.status_carry());

        assert_eq!(regs.status(), 0x2);
    }
}
