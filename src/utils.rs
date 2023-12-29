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
    fn test_parity() {
        assert_eq!(true, parity_even(0x0));
        assert_eq!(false, parity_even(0x1));
        assert_eq!(true, parity_even(0b11001100));
        assert_eq!(true, parity_even(0b11001001));
        assert_eq!(false, parity_even(0b10001100));
    }
}
