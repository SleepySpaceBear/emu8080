
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
