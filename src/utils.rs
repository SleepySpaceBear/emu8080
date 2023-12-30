pub fn check_carry(old_val: u8, new_val: u8) -> bool {
    if new_val < old_val {
        return true;
    }
    return false;
}

pub fn check_aux_carry(old_val: u8, new_val: u8) -> bool {
    if (new_val & 0xF) < (old_val & 0xF) {
        return true;
    }
    return false;
}

pub fn parity_even(val: u8) -> bool {
    let val: u8 = (0x0F & val) ^ (val >> 4);
    let val: u8 = (0x03 & val) ^ (val >> 2);
    let val: u8 = (0x01 & val) ^ (val >> 1);
    return val == 0;
}

pub fn twos_complement(val: u8) -> u8 {
    return (!val).wrapping_add(1)
}

pub fn make_u16(higher_order: u8, lower_order: u8) -> u16 {
    return ((higher_order as u16) << 8) | (lower_order as u16)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_carry() {
        assert_eq!(true, check_carry(0xFF, 0xFE));
        assert_eq!(false, check_carry(0x00, 0xCF));
    }

    #[test]
    fn test_aux_carry() {
        assert_eq!(true, check_aux_carry(0x01, 0xF0));
        assert_eq!(false, check_aux_carry(0x0E, 0xDE));
    }

    #[test]
    fn test_parity() {
        assert_eq!(true, parity_even(0x0));
        assert_eq!(false, parity_even(0x1));
        assert_eq!(true, parity_even(0b11001100));
        assert_eq!(true, parity_even(0b11001001));
        assert_eq!(false, parity_even(0b10001100));
    }

    #[test]
    fn test_make_u16() {
        assert_eq!(0xFFFF, make_u16(0xFF, 0xFF));
        assert_eq!(0xCC00, make_u16(0xCC, 0x00));
        assert_eq!(0x001A, make_u16(0x00, 0x1A));
    }
}
