use std::fmt;

use crate::memory::MemoryAccess as MemoryAccess;

// 2 MHz
pub const CYCLE_TIME_SECS: f64 = 0.000_000_005;
pub const CYCLE_TIME_NANO_SECS: u64 = 500;

#[allow(non_camel_case_types)]

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum Instruction {
    // No Operation
    NOP = 0x0,
    // Set Carry
    STC = 0x37,
    // Complement Carry
    CMC = 0x3f,
    // Increment Register or Memory 
    INR_B = 0x4,
    // Increment Register or Memory
    INR_C = 0xc,
    // Increment Register or Memory
    INR_D = 0x14,
    // Increment Register or Memory
    INR_E = 0x1c,
    // Increment Register or Memory
    INR_H = 0x24,
    // Increment Register or Memory
    INR_L = 0x2c,
    // Increment Register or Memory
    INR_M = 0x34,
    // Increment Register or Memory
    INR_A = 0x3c,
    // Decrement Register or Memory 
    DCR_B = 0x5,
    // Decrement Register or Memory
    DCR_C = 0xd,
    // Decrement Register or Memory
    DCR_D = 0x15,
    // Decrement Register or Memory
    DCR_E = 0x1d,
    // Decrement Register or Memory
    DCR_H = 0x25,
    // Decrement Register or Memory
    DCR_L = 0x2d,
    // Decrement Register or Memory
    DCR_M = 0x35,
    // Decrement Register or Memory
    DCR_A = 0x3d,
    // Complement Accumulator
    CMA = 0x2f,
    // Decimal Adjust Accumulator
    DAA = 0x27,
    // No Operation
    MOV_B_B = 0x40,
    // Move
    MOV_B_C = 0x41,
    // Move
    MOV_B_D = 0x42,
    // Move
    MOV_B_E = 0x43,
    // Move
    MOV_B_H = 0x44,
    // Move
    MOV_B_L = 0x45,
    // Move
    MOV_B_M = 0x46,
    // Move
    MOV_B_A = 0x47,
    // Move
    MOV_C_B = 0x48,
    // No Operation
    MOV_C_C = 0x49,
    // Move
    MOV_C_D = 0x4a,
    // Move
    MOV_C_E = 0x4b,
    // Move
    MOV_C_H = 0x4c,
    // Move
    MOV_C_L = 0x4d,
    // Move
    MOV_C_M = 0x4e,
    // Move
    MOV_C_A = 0x4f,
    // Move
    MOV_D_B = 0x50,
    // Move
    MOV_D_C = 0x51,
    // No Operation
    MOV_D_D = 0x52,
    // Move
    MOV_D_E = 0x53,
    // Move
    MOV_D_H = 0x54,
    // Move
    MOV_D_L = 0x55,
    // Move
    MOV_D_M = 0x56,
    // Move
    MOV_D_A = 0x57,
    // Move
    MOV_E_B = 0x58,
    // Move
    MOV_E_C = 0x59,
    // Move
    MOV_E_D = 0x5a,
    // No Operation
    MOV_E_E = 0x5b,
    // Move
    MOV_E_H = 0x5c,
    // Move
    MOV_E_L = 0x5d,
    // Move
    MOV_E_M = 0x5e,
    // Move
    MOV_E_A = 0x5f,
    // Move
    MOV_H_B = 0x60,
    // Move
    MOV_H_C = 0x61,
    // Move
    MOV_H_D = 0x62,
    // Move
    MOV_H_E = 0x63,
    // No Operation
    MOV_H_H = 0x64,
    // Move
    MOV_H_L = 0x65,
    // Move
    MOV_H_M = 0x66,
    // Move
    MOV_H_A = 0x67,
    // Move
    MOV_L_B = 0x68,
    // Move
    MOV_L_C = 0x69,
    // Move
    MOV_L_D = 0x6a,
    // Move
    MOV_L_E = 0x6b,
    // Move
    MOV_L_H = 0x6c,
    // No Operation
    MOV_L_L = 0x6d,
    // Move
    MOV_L_M = 0x6e,
    // Move
    MOV_L_A = 0x6f,
    // Move
    MOV_M_B = 0x70,
    // Move
    MOV_M_C = 0x71,
    // Move
    MOV_M_D = 0x72,
    // Move
    MOV_M_E = 0x73,
    // Move
    MOV_M_H = 0x74,
    // Move
    MOV_M_L = 0x75,
    // Move
    MOV_M_A = 0x77,
    // Move
    MOV_A_B = 0x78,
    // Move
    MOV_A_C = 0x79,
    // Move
    MOV_A_D = 0x7a,
    // Move
    MOV_A_E = 0x7b,
    // Move
    MOV_A_H = 0x7c,
    // Move
    MOV_A_L = 0x7d,
    // Move
    MOV_A_M = 0x7e,
    // No Operation
    MOV_A_A = 0x7f,
    // Store Accumulator
    STAX_B = 0x2,
    // Store Accumulator
    STAX_D = 0x12,
    // Load Accumulator
    LDAX_B = 0xa,
    // Load Accumulator
    LDAX_D = 0x1a,
    // ADD Register or Memory To Accumulator
    ADD_B = 0x80,
    // ADD Register or Memory To Accumulator
    ADD_C = 0x81,
    // ADD Register or Memory To Accumulator
    ADD_D = 0x82,
    // ADD Register or Memory To Accumulator
    ADD_E = 0x83,
    // ADD Register or Memory To Accumulator
    ADD_H = 0x84,
    // ADD Register or Memory To Accumulator
    ADD_L = 0x85,
    // ADD Register or Memory To Accumulator
    ADD_M = 0x86,
    // ADD Register or Memory To Accumulator
    ADD_A = 0x87,
    // ADD Register or Memory to Accumulator with Carry
    ADC_B = 0x88,
    // ADD Register or Memory to Accumulator with Carry
    ADC_C = 0x89,
    // ADD Register or Memory to Accumulator with Carry
    ADC_D = 0x8a,
    // ADD Register or Memory to Accumulator with Carry
    ADC_E = 0x8b,
    // ADD Register or Memory to Accumulator with Carry
    ADC_H = 0x8c,
    // ADD Register or Memory to Accumulator with Carry
    ADC_L = 0x8d,
    // ADD Register or Memory to Accumulator with Carry
    ADC_M = 0x8e,
    // ADD Register or Memory to Accumulator with Carry
    ADC_A = 0x8f,
    // Subtract Register or Memory From Accumulator
    SUB_B = 0x90,
    // Subtract Register or Memory From Accumulator
    SUB_C = 0x91,
    // Subtract Register or Memory From Accumulator
    SUB_D = 0x92,
    // Subtract Register or Memory From Accumulator
    SUB_E = 0x93,
    // Subtract Register or Memory From Accumulator
    SUB_H = 0x94,
    // Subtract Register or Memory From Accumulator
    SUB_L = 0x95,
    // Subtract Register or Memory From Accumulator
    SUB_M = 0x96,
    // Subtract Register or Memory From Accumulator
    SUB_A = 0x97,
    // Subtract Register or Memory From Accumulator With Borrow
    SBB_B = 0x98,
    // Subtract Register or Memory From Accumulator With Borrow
    SBB_C = 0x99,
    // Subtract Register or Memory From Accumulator With Borrow
    SBB_D = 0x9a,
    // Subtract Register or Memory From Accumulator With Borrow
    SBB_E = 0x9b,
    // Subtract Register or Memory From Accumulator With Borrow
    SBB_H = 0x9c,
    // Subtract Register or Memory From Accumulator With Borrow
    SBB_L = 0x9d,
    // Subtract Register or Memory From Accumulator With Borrow
    SBB_M = 0x9e,
    // Subtract Register or Memory From Accumulator With Borrow
    SBB_A = 0x9f,
    // Logical and Register or Memory With Accumulator
    ANA_B = 0xa0,
    // Logical and Register or Memory With Accumulator
    ANA_C = 0xa1,
    // Logical and Register or Memory With Accumulator
    ANA_D = 0xa2,
    // Logical and Register or Memory With Accumulator
    ANA_E = 0xa3,
    // Logical and Register or Memory With Accumulator
    ANA_H = 0xa4,
    // Logical and Register or Memory With Accumulator
    ANA_L = 0xa5,
    // Logical and Register or Memory With Accumulator
    ANA_M = 0xa6,
    // Logical and Register or Memory With Accumulator
    ANA_A = 0xa7,
    // Logical Exclusive-Or Register or Memory With Accumulator (Zero Accumulator)
    XRA_B = 0xa8,
    // Logical Exclusive-Or Register or Memory With Accumulator (Zero Accumulator)
    XRA_C = 0xa9,
    // Logical Exclusive-Or Register or Memory With Accumulator (Zero Accumulator)
    XRA_D = 0xaa,
    // Logical Exclusive-Or Register or Memory With Accumulator (Zero Accumulator)
    XRA_E = 0xab,
    // Logical Exclusive-Or Register or Memory With Accumulator (Zero Accumulator)
    XRA_H = 0xac,
    // Logical Exclusive-Or Register or Memory With Accumulator (Zero Accumulator)
    XRA_L = 0xad,
    // Logical Exclusive-Or Register or Memory With Accumulator (Zero Accumulator)
    XRA_M = 0xae,
    // Logical Exclusive-Or Register or Memory With Accumulator (Zero Accumulator)
    XRA_A = 0xaf,
    // Logical or Register or Memory With Accumulator
    ORA_B = 0xb0,
    // Logical or Register or Memory With Accumulator
    ORA_C = 0xb1,
    // Logical or Register or Memory With Accumulator
    ORA_D = 0xb2,
    // Logical or Register or Memory With Accumulator
    ORA_E = 0xb3,
    // Logical or Register or Memory With Accumulator
    ORA_H = 0xb4,
    // Logical or Register or Memory With Accumulator
    ORA_L = 0xb5,
    // Logical or Register or Memory With Accumulator
    ORA_M = 0xb6,
    // Logical or Register or Memory With Accumulator
    ORA_A = 0xb7,
    // Compare Register or Memory With Accumulator
    CMP_B = 0xb8,
    // Compare Register or Memory With Accumulator
    CMP_C = 0xb9,
    // Compare Register or Memory With Accumulator
    CMP_D = 0xba,
    // Compare Register or Memory With Accumulator
    CMP_E = 0xbb,
    // Compare Register or Memory With Accumulator
    CMP_H = 0xbc,
    // Compare Register or Memory With Accumulator
    CMP_L = 0xbd,
    // Compare Register or Memory With Accumulator
    CMP_M = 0xbe,
    // Compare Register or Memory With Accumulator
    CMP_A = 0xbf,
    // Rotate Accumulator Left
    RLC = 0x7,
    // Rotate Accumulator Right
    RRC = 0xf,
    // Rotate Accumulator Left Through Carry
    RAL = 0x17,
    // Rotate Accumulator Right Through Carry
    RAR = 0x1f,
    // Push Data Onto Stack
    PUSH_B = 0xc5,
    // Push Data Onto Stack
    PUSH_D = 0xd5,
    // Push Data Onto Stack
    PUSH_H = 0xe5,
    // Push Data Onto Stack
    PUSH_PSW = 0xf5,
    // Pop Data Off Stack
    POP_B = 0xc1,
    // Pop Data Off Stack
    POP_D = 0xd1,
    // Pop Data Off Stack
    POP_H = 0xe1,
    // Pop Data Off Stack
    POP_PSW = 0xf1,
    // Double Add
    DAD_B = 0x9,
    // Double Add
    DAD_D = 0x19,
    // Double Add
    DAD_H = 0x29,
    // Double Add
    DAD_SP = 0x39,
    // Increment Register Pair
    INX_B = 0x3,
    // Increment Register Pair
    INX_D = 0x13,
    // Increment Register Pair
    INX_H = 0x23,
    // Increment Register Pair
    INX_SP = 0x33,
    // Decrement Register Pair
    DCX_B = 0xb,
    // Decrement Register Pair
    DCX_D = 0x1b,
    // Decrement Register Pair
    DCX_H = 0x2b,
    // Decrement Register Pair
    DCX_SP = 0x3b,
    // Exchange Registers
    XCHG = 0xeb,
    // Exchange Stack
    XTHL = 0xe3,
    // Load SP From H and L
    SPHL = 0xf9,
    // Move Immediate Data
    LXI_B = 0x1,
    // Move Immediate Data
    LXI_D = 0x11,
    // Move Immediate Data
    LXI_H = 0x21,
    // Move Immediate Data
    LXI_SP = 0x31,
    // Move Immediate Data
    MVI_B = 0x6,
    // Move Immediate Data
    MVI_C = 0xe,
    // Move Immediate Data
    MVI_D = 0x16,
    // Move Immediate Data
    MVI_E = 0x1e,
    // Move Immediate Data
    MVI_H = 0x26,
    // Move Immediate Data
    MVI_L = 0x2e,
    // Move Immediate Data
    MVI_M = 0x36,
    // Move Immediate Data
    MVI_A = 0x3e,
    // Add Immediate to Accumulator
    ADI = 0xc6,
    // Add Immediate to Accumulator With Carry
    ACI = 0xce,
    // Subtract Immediate from Accumulator
    SUI = 0xd6,
    // Subtract Immediate from Accumulator With Borrow
    SBI = 0xde,
    // And Immediate With Accumulator
    ANI = 0xe6,
    // Exclusive-Or Immediate With Accumulator
    XRI = 0xee,
    // Or Immediate With Accumulator 
    ORI = 0xf6,
    // Compare Immediate With Accumulator
    CPI = 0xfe,
    // Store Accumulator Direct
    STA = 0x32,
    // Load Accumulator Direct
    LDA = 0x3a,
    // Store H and L Direct
    SHLD = 0x22,
    // Load H and L Direct
    LHLD = 0x2a,
    // Load Program Counter
    PCHL = 0xe9,
    // Jump
    JMP = 0xc3,
    // Jump If Carry
    JC = 0xda,
    // Jump No Carry
    JNC = 0xd2,
    // Jump If Zero
    JZ = 0xca,
    // Jump If Not Zero
    JNZ = 0xc2,
    // Jump If Minus
    JM = 0xfa,
    // Jump If Positive
    JP = 0xf2,
    // Jump If Parity Even
    JPE = 0xea,
    // Jump If Parity Odd
    JPO = 0xe2,
    // Call
    CALL = 0xcd,
    // Call If Carry
    CC = 0xdc,
    // Call If No Carry
    CNC = 0xd4,
    // Call If Zero
    CZ = 0xcc,
    // Call If Not Zero
    CNZ = 0xc4,
    // Call If Minus
    CM = 0xfc,
    // Call If Plus
    CP = 0xf4,
    // Call If Parity Even
    CPE = 0xec,
    // Call If Parity Odd
    CPO = 0xe4,
    // Return
    RET = 0xc9,
    // Return If Carry
    RC = 0xd8,
    // Return If No Carry
    RNC = 0xd0,
    // Return If Zero
    RZ = 0xc8,
    // Return If Not Zero
    RNZ = 0xc0,
    // Return If Minus
    RM = 0xf8,
    // Return If Plus
    RP = 0xf0,
    // Return If Parity Even
    RPE = 0xe8,
    // Return If Parity Odd
    RPO = 0xe0,
    // Restart
    RST_1 = 0xc7,
    // Restart
    RST_2 = 0xcf,
    // Restart
    RST_3 = 0xd7,
    // Restart
    RST_4 = 0xdf,
    // Restart
    RST_5 = 0xe7,
    // Restart
    RST_6 = 0xef,
    // Restart
    RST_7 = 0xf7,
    // Restart
    RST_8 = 0xff,
    // Enable Interrupts
    EI = 0xfb,
    // Disable Interrupts
    DI = 0xf3,
    // Input
    IN = 0xdb,
    // Output
    OUT = 0xd3,
    // Halt
    HLT = 0x76,
}

impl From<u8> for Instruction {
    fn from(orig: u8) -> Self {
        return match orig {
            0x00 | 0x10 | 0x20 | 0x30 | 0x08 | 0x18 | 0x28 | 0x38 => Instruction::NOP,
            0x37 => Instruction::STC,
            0x3f => Instruction::CMC,
            0x4 => Instruction::INR_B,
            0xc => Instruction::INR_C,
            0x14 => Instruction::INR_D,
            0x1c => Instruction::INR_E,
            0x24 => Instruction::INR_H,
            0x2c => Instruction::INR_L,
            0x34 => Instruction::INR_M,
            0x3c => Instruction::INR_A,
            0x5 => Instruction::DCR_B,
            0xd => Instruction::DCR_C,
            0x15 => Instruction::DCR_D,
            0x1d => Instruction::DCR_E,
            0x25 => Instruction::DCR_H,
            0x2d => Instruction::DCR_L,
            0x35 => Instruction::DCR_M,
            0x3d => Instruction::DCR_A,
            0x2f => Instruction::CMA,
            0x27 => Instruction::DAA,
            0x40 => Instruction::MOV_B_B,
            0x41 => Instruction::MOV_B_C,
            0x42 => Instruction::MOV_B_D,
            0x43 => Instruction::MOV_B_E,
            0x44 => Instruction::MOV_B_H,
            0x45 => Instruction::MOV_B_L,
            0x46 => Instruction::MOV_B_M,
            0x47 => Instruction::MOV_B_A,
            0x48 => Instruction::MOV_C_B,
            0x49 => Instruction::MOV_C_C,
            0x4a => Instruction::MOV_C_D,
            0x4b => Instruction::MOV_C_E,
            0x4c => Instruction::MOV_C_H,
            0x4d => Instruction::MOV_C_L,
            0x4e => Instruction::MOV_C_M,
            0x4f => Instruction::MOV_C_A,
            0x50 => Instruction::MOV_D_B,
            0x51 => Instruction::MOV_D_C,
            0x52 => Instruction::MOV_D_D,
            0x53 => Instruction::MOV_D_E,
            0x54 => Instruction::MOV_D_H,
            0x55 => Instruction::MOV_D_L,
            0x56 => Instruction::MOV_D_M,
            0x57 => Instruction::MOV_D_A,
            0x58 => Instruction::MOV_E_B,
            0x59 => Instruction::MOV_E_C,
            0x5a => Instruction::MOV_E_D,
            0x5b => Instruction::MOV_E_E,
            0x5c => Instruction::MOV_E_H,
            0x5d => Instruction::MOV_E_L,
            0x5e => Instruction::MOV_E_M,
            0x5f => Instruction::MOV_E_A,
            0x60 => Instruction::MOV_H_B,
            0x61 => Instruction::MOV_H_C,
            0x62 => Instruction::MOV_H_D,
            0x63 => Instruction::MOV_H_E,
            0x64 => Instruction::MOV_H_H,
            0x65 => Instruction::MOV_H_L,
            0x66 => Instruction::MOV_H_M,
            0x67 => Instruction::MOV_H_A,
            0x68 => Instruction::MOV_L_B,
            0x69 => Instruction::MOV_L_C,
            0x6a => Instruction::MOV_L_D,
            0x6b => Instruction::MOV_L_E,
            0x6c => Instruction::MOV_L_H,
            0x6d => Instruction::MOV_L_L,
            0x6e => Instruction::MOV_L_M,
            0x6f => Instruction::MOV_L_A,
            0x70 => Instruction::MOV_M_B,
            0x71 => Instruction::MOV_M_C,
            0x72 => Instruction::MOV_M_D,
            0x73 => Instruction::MOV_M_E,
            0x74 => Instruction::MOV_M_H,
            0x75 => Instruction::MOV_M_L,
            0x77 => Instruction::MOV_M_A,
            0x78 => Instruction::MOV_A_B,
            0x79 => Instruction::MOV_A_C,
            0x7a => Instruction::MOV_A_D,
            0x7b => Instruction::MOV_A_E,
            0x7c => Instruction::MOV_A_H,
            0x7d => Instruction::MOV_A_L,
            0x7e => Instruction::MOV_A_M,
            0x7f => Instruction::MOV_A_A,
            0x02 => Instruction::STAX_B,
            0x12 => Instruction::STAX_D,
            0x0a => Instruction::LDAX_B,
            0x1a => Instruction::LDAX_D,
            0x80 => Instruction::ADD_B,
            0x81 => Instruction::ADD_C,
            0x82 => Instruction::ADD_D,
            0x83 => Instruction::ADD_E,
            0x84 => Instruction::ADD_H,
            0x85 => Instruction::ADD_L,
            0x86 => Instruction::ADD_M,
            0x87 => Instruction::ADD_A,
            0x88 => Instruction::ADC_B,
            0x89 => Instruction::ADC_C,
            0x8a => Instruction::ADC_D,
            0x8b => Instruction::ADC_E,
            0x8c => Instruction::ADC_H,
            0x8d => Instruction::ADC_L,
            0x8e => Instruction::ADC_M,
            0x8f => Instruction::ADC_A,
            0x90 => Instruction::SUB_B,
            0x91 => Instruction::SUB_C,
            0x92 => Instruction::SUB_D,
            0x93 => Instruction::SUB_E,
            0x94 => Instruction::SUB_H,
            0x95 => Instruction::SUB_L,
            0x96 => Instruction::SUB_M,
            0x97 => Instruction::SUB_A,
            0x98 => Instruction::SBB_B,
            0x99 => Instruction::SBB_C,
            0x9a => Instruction::SBB_D,
            0x9b => Instruction::SBB_E,
            0x9c => Instruction::SBB_H,
            0x9d => Instruction::SBB_L,
            0x9e => Instruction::SBB_M,
            0x9f => Instruction::SBB_A,
            0xa0 => Instruction::ANA_B,
            0xa1 => Instruction::ANA_C,
            0xa2 => Instruction::ANA_D,
            0xa3 => Instruction::ANA_E,
            0xa4 => Instruction::ANA_H,
            0xa5 => Instruction::ANA_L,
            0xa6 => Instruction::ANA_M,
            0xa7 => Instruction::ANA_A,
            0xa8 => Instruction::XRA_B,
            0xa9 => Instruction::XRA_C,
            0xaa => Instruction::XRA_D,
            0xab => Instruction::XRA_E,
            0xac => Instruction::XRA_H,
            0xad => Instruction::XRA_L,
            0xae => Instruction::XRA_M,
            0xaf => Instruction::XRA_A,
            0xb0 => Instruction::ORA_B,
            0xb1 => Instruction::ORA_C,
            0xb2 => Instruction::ORA_D,
            0xb3 => Instruction::ORA_E,
            0xb4 => Instruction::ORA_H,
            0xb5 => Instruction::ORA_L,
            0xb6 => Instruction::ORA_M,
            0xb7 => Instruction::ORA_A,
            0xb8 => Instruction::CMP_B,
            0xb9 => Instruction::CMP_C,
            0xba => Instruction::CMP_D,
            0xbb => Instruction::CMP_E,
            0xbc => Instruction::CMP_H,
            0xbd => Instruction::CMP_L,
            0xbe => Instruction::CMP_M,
            0xbf => Instruction::CMP_A,
            0x07 => Instruction::RLC,
            0x0f => Instruction::RRC,
            0x17 => Instruction::RAL,
            0x1f => Instruction::RAR,
            0xc5 => Instruction::PUSH_B,
            0xd5 => Instruction::PUSH_D,
            0xe5 => Instruction::PUSH_H,
            0xf5 => Instruction::PUSH_PSW,
            0xc1 => Instruction::POP_B,
            0xd1 => Instruction::POP_D,
            0xe1 => Instruction::POP_H,
            0xf1 => Instruction::POP_PSW,
            0x09 => Instruction::DAD_B,
            0x19 => Instruction::DAD_D,
            0x29 => Instruction::DAD_H,
            0x39 => Instruction::DAD_SP,
            0x03 => Instruction::INX_B,
            0x13 => Instruction::INX_D,
            0x23 => Instruction::INX_H,
            0x33 => Instruction::INX_SP,
            0x0b => Instruction::DCX_B,
            0x1b => Instruction::DCX_D,
            0x2b => Instruction::DCX_H,
            0x3b => Instruction::DCX_SP,
            0xeb => Instruction::XCHG,
            0xe3 => Instruction::XTHL,
            0xf9 => Instruction::SPHL,
            0x01 => Instruction::LXI_B,
            0x11 => Instruction::LXI_D,
            0x21 => Instruction::LXI_H,
            0x31 => Instruction::LXI_SP,
            0x06 => Instruction::MVI_B,
            0x0e => Instruction::MVI_C,
            0x16 => Instruction::MVI_D,
            0x1e => Instruction::MVI_E,
            0x26 => Instruction::MVI_H,
            0x2e => Instruction::MVI_L,
            0x36 => Instruction::MVI_M,
            0x3e => Instruction::MVI_A,
            0xc6 => Instruction::ADI,
            0xce => Instruction::ACI,
            0xd6 => Instruction::SUI,
            0xde => Instruction::SBI,
            0xe6 => Instruction::ANI,
            0xee => Instruction::XRI,
            0xf6 => Instruction::ORI,
            0xfe => Instruction::CPI,
            0x32 => Instruction::STA,
            0x3a => Instruction::LDA,
            0x22 => Instruction::SHLD,
            0x2a => Instruction::LHLD,
            0xe9 => Instruction::PCHL,
            0xc3 | 0xcb => Instruction::JMP,
            0xda => Instruction::JC,
            0xd2 => Instruction::JNC,
            0xca => Instruction::JZ,
            0xc2 => Instruction::JNZ,
            0xfa => Instruction::JM,
            0xf2 => Instruction::JP,
            0xea => Instruction::JPE,
            0xe2 => Instruction::JPO,
            0xcd | 0xdd | 0xed | 0xfd => Instruction::CALL,
            0xdc => Instruction::CC,
            0xd4 => Instruction::CNC,
            0xcc => Instruction::CZ,
            0xc4 => Instruction::CNZ,
            0xfc => Instruction::CM,
            0xf4 => Instruction::CP,
            0xec => Instruction::CPE,
            0xe4 => Instruction::CPO,
            0xc9 | 0xd9 => Instruction::RET,
            0xd8 => Instruction::RC,
            0xd0 => Instruction::RNC,
            0xc8 => Instruction::RZ,
            0xc0 => Instruction::RNZ,
            0xf8 => Instruction::RM,
            0xf0 => Instruction::RP,
            0xe8 => Instruction::RPE,
            0xe0 => Instruction::RPO,
            0xc7 => Instruction::RST_1,
            0xcf => Instruction::RST_2,
            0xd7 => Instruction::RST_3,
            0xdf => Instruction::RST_4,
            0xe7 => Instruction::RST_5,
            0xef => Instruction::RST_6,
            0xf7 => Instruction::RST_7,
            0xff => Instruction::RST_8,
            0xfb => Instruction::EI,
            0xf3 => Instruction::DI,
            0xdb => Instruction::IN,
            0xd3 => Instruction::OUT,
            0x76 => Instruction::HLT
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
enum StatusFlags {
    SignBit = 0x80,
    ZeroBit = 0x40,
    AuxCarryBit = 0x10,
    ParityBit = 0x04,
    CarryBit = 0x01
}

struct Registers {
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
    fn new() -> Self {
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
            status: 0b0000_0010,
            w: 0,
            z: 0
        }
    }

    fn pc(&self) -> u16 {
        self.pc
    }

    fn set_pc(&mut self, val: u16) {
        self.pc = val
    }


    fn sp(&self) -> u16 {
        self.sp
    }

    fn set_sp(&mut self, val: u16) {
        self.sp = val
    }


    fn pair_b(&self) -> u16 {
        make_u16(self.b, self.c)
    }

    fn set_pair_b(&mut self, val: u16) {
       self.b = ((val >> 8) & 0xFF) as u8;
       self.c = (val & 0xFF) as u8
    }

    fn b(&self) -> u8 {
        self.b
    }

    fn set_b(&mut self, val: u8) {
        self.b = val 
    }

    fn c(&self) -> u8 {
        self.c 
    }

    fn set_c(&mut self, val: u8) {
        self.c = val 
    }


    fn pair_d(&self) -> u16 {
        make_u16(self.d, self.e)
    }

    fn set_pair_d(&mut self, val: u16) {
       self.d = ((val >> 8) & 0xFF) as u8;
       self.e = (val & 0xFF) as u8
    }

    fn d(&self) -> u8 {
        self.d 
    }

    fn set_d(&mut self, val: u8) {
        self.d = val 
    }

    fn e(&self) -> u8 {
        self.e 
    }

    fn set_e(&mut self, val: u8) {
        self.e = val 
    }


    fn pair_h(&self) -> u16 {
        make_u16(self.h, self.l)
    }

    fn set_pair_h(&mut self, val: u16) {
       self.h = ((val >> 8) & 0xFF) as u8;
       self.l = (val & 0xFF) as u8
    } 

    fn h(&self) -> u8 {
        self.h 
    }

    fn set_h(&mut self, val: u8) {
        self.h = val 
    }

    fn l(&self) -> u8 {
        self.l 
    }

    fn set_l(&mut self, val: u8) {
        self.l = val
    }


    fn psw(&self) -> u16 {
        make_u16(self.accumulator, self.status)
    }

    fn set_psw(&mut self, val: u16) {
        self.accumulator = ((val >> 8) & 0xFF) as u8;
        self.set_status((val & 0xFF) as u8);
    }

    fn accumulator(&self) -> u8 {
        self.accumulator 
    }

    fn set_accumulator(&mut self, val: u8) {
        self.accumulator = val
    }

    fn status(&self) -> u8 {
        self.status
    }

    fn set_status(&mut self, val: u8) {
        self.status = 0b0000_0010 | (val & 0b1101_0111);
    }

    fn status_carry(&self) -> bool {
        return (self.status & (StatusFlags::CarryBit as u8)) != 0; 
    }

    fn set_status_carry(&mut self, carry: bool) {
        if carry {
            self.status |= StatusFlags::CarryBit as u8;
        }
        else {
            self.status &= !(StatusFlags::CarryBit as u8);
        }
    }
    
    fn status_aux_carry(&self) -> bool {
        return (self.status & (StatusFlags::AuxCarryBit as u8)) != 0; 
    }

    fn set_status_aux_carry(&mut self, aux_carry: bool) {
        if aux_carry {
            self.status |= StatusFlags::AuxCarryBit as u8;
        }
        else {
            self.status &= !(StatusFlags::AuxCarryBit as u8);
        }
    }

    fn status_zero(&self) -> bool {
        return (self.status & StatusFlags::ZeroBit as u8) != 0; 
    }

    fn set_status_zero(&mut self, zero: bool) {
        if zero {
            self.status |= StatusFlags::ZeroBit as u8;
        }
        else {
            self.status &= !(StatusFlags::ZeroBit as u8);
        }
    }
    
    fn status_parity(&self) -> bool {
        return (self.status & (StatusFlags::ParityBit as u8)) != 0; 
    }

    fn set_status_parity(&mut self, parity: bool) {
        if parity {
            self.status |= StatusFlags::ParityBit as u8;
        }
        else {
            self.status &= !(StatusFlags::ParityBit as u8);
        }
    }
    
    fn status_sign(&self) -> bool {
        return (self.status & (StatusFlags::SignBit as u8)) != 0; 
    }

    fn set_status_sign(&mut self, sign: bool) {
        if sign {
            self.status |= StatusFlags::SignBit as u8;
        }
        else {
            self.status &= !(StatusFlags::SignBit as u8);
        }
    }

    fn w(&self) -> u8 {
       self.w 
    }

    fn set_w(&mut self, val: u8) {
        self.w = val;
    }
    
    pub fn z(&self) -> u8 {
       self.z 
    }

    fn set_z(&mut self, val: u8) {
        self.z = val;
    }

    fn pair_w(&self) -> u16 {
       make_u16(self.w(), self.z())
    }
}

impl fmt::Debug for Registers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Registers [B,C: {:02X}, {:02X}; D,E: {:02X}, {:02X};
                   H,L: {:02X}, {:02X}; PSW: {:02X}, {:02X}; PC: {:04X}; SP: {:04X}]",
            self.b, self.c, self.d, self.e, 
            self.h, self.l, self.accumulator, self.status, self.pc, self.sp)
    }
}


#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
enum Operand8 {
    RegB = 0,
    RegC = 1,
    RegD = 2,
    RegE = 3,
    RegH = 4,
    RegL = 5,
    Memory = 6,
    RegA = 7,
    Immediate = 8
}

impl From<u8> for Operand8 {
    fn from(orig: u8) -> Self {
        match orig {
            0 => return Operand8::RegB,
            1 => return Operand8::RegC,
            2 => return Operand8::RegD,
            3 => return Operand8::RegE,
            4 => return Operand8::RegH,
            5 => return Operand8::RegL,
            6 => return Operand8::Memory,
            7 => return Operand8::RegA,
            _ => return Operand8::Immediate
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
enum Operand16 {
    RegPairB = 0,
    RegPairD = 1,
    RegPairH = 2,
    PSW = 3,
    SP = 4
}

impl From<u8> for Operand16 {
    fn from(orig: u8) -> Self {
        match orig {
            0 => Operand16::RegPairB,
            1 => Operand16::RegPairD,
            2 => Operand16::RegPairH,
            3 => Operand16::PSW,
            _ => Operand16::SP
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
enum ConditionCode {
    NotZero = 0,
    Zero = 1,
    NoCarry = 2,
    Carry = 3,
    ParityOdd = 4,
    ParityEven = 5,
    Postive = 6,
    Minus = 7,
    Unconditional = 8
}

#[derive(Debug)]
pub struct Intel8080 {
    registers: Registers,
    interrupt_instruction: Option<Instruction>,
    io_port: u8,
    stopped: bool,
    output_ready: bool,
    awaiting_input: bool,
    inte: bool
}

impl Intel8080 {
    pub fn new() -> Self {
        Self {
            registers: Registers::new(),
            interrupt_instruction: None,
            io_port: 0,
            stopped: false,
            output_ready: false,
            awaiting_input: false,
            inte: false,
        }
    }
    
    pub fn active_io_port(&self) -> u8 {
        self.io_port
    }

    pub fn awaiting_input(&self) -> bool {
        self.awaiting_input
    }

    pub fn write_input(&mut self, data: u8) {
        if self.awaiting_input {
            self.registers.set_accumulator(data);
        }
    }

    pub fn output_ready(&self) -> bool {
        self.output_ready
    }

    pub fn read_output(&self) -> u8 {
        match self.output_ready {
            true => self.registers.accumulator(),
            false => 0
        }
    }

    pub fn step(&mut self, memory: &mut impl MemoryAccess) -> u64 {
        self.awaiting_input = false;
        self.output_ready = false;
        
        if !self.stopped {
            
            let instruction = match self.interrupt_instruction.take() {
                Some(instruction) => instruction,
                None => self.fetch_instruction(memory)
            };
            return self.do_instruction(instruction, memory)
        }
        else {
            return 0
        }
    }

    pub fn interrupt(&mut self, instruction: Instruction) {
        if self.inte {
            self.interrupt_instruction = Some(instruction);
            self.stopped = false;
        }
    }

    pub fn reset(&mut self) {
        self.registers.set_pc(0);
    }

    fn fetch_instruction(&mut self, memory: &impl MemoryAccess) -> Instruction {
        self.registers.set_pc(self.registers.pc().wrapping_add(1));
        Instruction::from(memory.read_byte(self.registers.pc().wrapping_sub(1)))
    }

    fn fetch_immediate(&mut self, memory: &impl MemoryAccess) -> u8 {
        self.registers.set_pc(self.registers.pc().wrapping_add(1));
        memory.read_byte(self.registers.pc().wrapping_sub(1))
    }


    fn do_instruction(&mut self, instruction: Instruction, memory: &mut impl MemoryAccess) -> u64 {
        return match instruction {
            Instruction::STC => { self.stc() },
            Instruction::CMC => { self.cmc() },
            Instruction::INR_B => { self.inr(Operand8::RegB, memory) },
            Instruction::INR_C => { self.inr(Operand8::RegC, memory) },
            Instruction::INR_D => { self.inr(Operand8::RegD, memory) },
            Instruction::INR_E => { self.inr(Operand8::RegE, memory) },
            Instruction::INR_H => { self.inr(Operand8::RegH, memory) },
            Instruction::INR_L => { self.inr(Operand8::RegL, memory) },
            Instruction::INR_M => { self.inr(Operand8::Memory, memory) },
            Instruction::INR_A => { self.inr(Operand8::RegA, memory) },
            Instruction::DCR_B => { self.dcr(Operand8::RegB, memory) },
            Instruction::DCR_C => { self.dcr(Operand8::RegC, memory) },
            Instruction::DCR_D => { self.dcr(Operand8::RegD, memory) },
            Instruction::DCR_E => { self.dcr(Operand8::RegE, memory) },
            Instruction::DCR_H => { self.dcr(Operand8::RegH, memory) },
            Instruction::DCR_L => { self.dcr(Operand8::RegL, memory) },
            Instruction::DCR_M => { self.dcr(Operand8::Memory, memory) },
            Instruction::DCR_A => { self.dcr(Operand8::RegA, memory) },
            Instruction::CMA => { self.cma() },
            Instruction::DAA => { self.daa() },
            Instruction::MOV_B_C => { self.mov(Operand8::RegB, Operand8::RegC, memory) },
            Instruction::MOV_B_D => { self.mov(Operand8::RegB, Operand8::RegD, memory) },
            Instruction::MOV_B_E => { self.mov(Operand8::RegB, Operand8::RegE, memory) },
            Instruction::MOV_B_H => { self.mov(Operand8::RegB, Operand8::RegH, memory) },
            Instruction::MOV_B_L => { self.mov(Operand8::RegB, Operand8::RegL, memory) },
            Instruction::MOV_B_M => { self.mov(Operand8::RegB, Operand8::Memory, memory) },
            Instruction::MOV_B_A => { self.mov(Operand8::RegB, Operand8::RegA, memory) },
            Instruction::MOV_C_B => { self.mov(Operand8::RegC, Operand8::RegB, memory) },
            Instruction::MOV_C_D => { self.mov(Operand8::RegC, Operand8::RegD, memory) },
            Instruction::MOV_C_E => { self.mov(Operand8::RegC, Operand8::RegE, memory) },
            Instruction::MOV_C_H => { self.mov(Operand8::RegC, Operand8::RegH, memory) },
            Instruction::MOV_C_L => { self.mov(Operand8::RegC, Operand8::RegL, memory) },
            Instruction::MOV_C_M => { self.mov(Operand8::RegC, Operand8::Memory, memory) },
            Instruction::MOV_C_A => { self.mov(Operand8::RegC, Operand8::RegA, memory) },
            Instruction::MOV_D_B => { self.mov(Operand8::RegD, Operand8::RegB, memory) },
            Instruction::MOV_D_C => { self.mov(Operand8::RegD, Operand8::RegC, memory) },
            Instruction::MOV_D_E => { self.mov(Operand8::RegD, Operand8::RegE, memory) },
            Instruction::MOV_D_H => { self.mov(Operand8::RegD, Operand8::RegH, memory) },
            Instruction::MOV_D_L => { self.mov(Operand8::RegD, Operand8::RegL, memory) },
            Instruction::MOV_D_M => { self.mov(Operand8::RegD, Operand8::Memory, memory) },
            Instruction::MOV_D_A => { self.mov(Operand8::RegD, Operand8::RegA, memory) },
            Instruction::MOV_E_B => { self.mov(Operand8::RegE, Operand8::RegB, memory) },
            Instruction::MOV_E_C => { self.mov(Operand8::RegE, Operand8::RegC, memory) },
            Instruction::MOV_E_D => { self.mov(Operand8::RegE, Operand8::RegD, memory) },
            Instruction::MOV_E_H => { self.mov(Operand8::RegE, Operand8::RegH, memory) },
            Instruction::MOV_E_L => { self.mov(Operand8::RegE, Operand8::RegL, memory) },
            Instruction::MOV_E_M => { self.mov(Operand8::RegE, Operand8::Memory, memory) },
            Instruction::MOV_E_A => { self.mov(Operand8::RegE, Operand8::RegA, memory) },
            Instruction::MOV_H_B => { self.mov(Operand8::RegH, Operand8::RegB, memory) },
            Instruction::MOV_H_C => { self.mov(Operand8::RegH, Operand8::RegC, memory) },
            Instruction::MOV_H_D => { self.mov(Operand8::RegH, Operand8::RegD, memory) },
            Instruction::MOV_H_E => { self.mov(Operand8::RegE, Operand8::RegE, memory) },
            Instruction::MOV_H_L => { self.mov(Operand8::RegH, Operand8::RegL, memory) },
            Instruction::MOV_H_M => { self.mov(Operand8::RegH, Operand8::Memory, memory) },
            Instruction::MOV_H_A => { self.mov(Operand8::RegH, Operand8::RegA, memory) },
            Instruction::MOV_L_B => { self.mov(Operand8::RegL, Operand8::RegB, memory) },
            Instruction::MOV_L_C => { self.mov(Operand8::RegL, Operand8::RegC, memory) },
            Instruction::MOV_L_D => { self.mov(Operand8::RegL, Operand8::RegD, memory) },
            Instruction::MOV_L_E => { self.mov(Operand8::RegL, Operand8::RegE, memory) },
            Instruction::MOV_L_H => { self.mov(Operand8::RegL, Operand8::RegH, memory) },
            Instruction::MOV_L_M => { self.mov(Operand8::RegL, Operand8::Memory, memory) },
            Instruction::MOV_L_A => { self.mov(Operand8::RegL, Operand8::RegA, memory) },
            Instruction::MOV_M_B => { self.mov(Operand8::Memory, Operand8::RegB, memory) },
            Instruction::MOV_M_C => { self.mov(Operand8::Memory, Operand8::RegC, memory) },
            Instruction::MOV_M_D => { self.mov(Operand8::Memory, Operand8::RegD, memory) },
            Instruction::MOV_M_E => { self.mov(Operand8::Memory, Operand8::RegE, memory) },
            Instruction::MOV_M_H => { self.mov(Operand8::Memory, Operand8::RegH, memory) },
            Instruction::MOV_M_L => { self.mov(Operand8::Memory, Operand8::RegL, memory) },
            Instruction::MOV_M_A => { self.mov(Operand8::Memory, Operand8::RegA, memory) },
            Instruction::MOV_A_B => { self.mov(Operand8::RegA, Operand8::RegB, memory) },
            Instruction::MOV_A_C => { self.mov(Operand8::RegA, Operand8::RegC, memory) },
            Instruction::MOV_A_D => { self.mov(Operand8::RegA, Operand8::RegD, memory) },
            Instruction::MOV_A_E => { self.mov(Operand8::RegA, Operand8::RegE, memory) },
            Instruction::MOV_A_H => { self.mov(Operand8::RegA, Operand8::RegH, memory) },
            Instruction::MOV_A_L => { self.mov(Operand8::RegA, Operand8::RegL, memory) },
            Instruction::MOV_A_M => { self.mov(Operand8::RegA, Operand8::Memory, memory) },
            Instruction::STAX_B => { self.stax(Operand16::RegPairB, memory) },
            Instruction::STAX_D => { self.stax(Operand16::RegPairD, memory) },
            Instruction::LDAX_B => { self.ldax(Operand16::RegPairB, memory) },
            Instruction::LDAX_D => { self.ldax(Operand16::RegPairD, memory) },
            Instruction::ADD_B => { self.add(Operand8::RegB, memory) },
            Instruction::ADD_C => { self.add(Operand8::RegC, memory) },
            Instruction::ADD_D => { self.add(Operand8::RegD, memory) },
            Instruction::ADD_E => { self.add(Operand8::RegE, memory) },
            Instruction::ADD_H => { self.add(Operand8::RegH, memory) },
            Instruction::ADD_L => { self.add(Operand8::RegL, memory) },
            Instruction::ADD_M => { self.add(Operand8::Memory, memory) },
            Instruction::ADD_A => { self.add(Operand8::RegA, memory) },
            Instruction::ADC_B => { self.adc(Operand8::RegB, memory) },
            Instruction::ADC_C => { self.adc(Operand8::RegC, memory) },
            Instruction::ADC_D => { self.adc(Operand8::RegD, memory) },
            Instruction::ADC_E => { self.adc(Operand8::RegE, memory) },
            Instruction::ADC_H => { self.adc(Operand8::RegH, memory) },
            Instruction::ADC_L => { self.adc(Operand8::RegL, memory) },
            Instruction::ADC_M => { self.adc(Operand8::Memory, memory) },
            Instruction::ADC_A => { self.adc(Operand8::RegA, memory) },
            Instruction::SUB_B => { self.sub(Operand8::RegB, memory) },
            Instruction::SUB_C => { self.sub(Operand8::RegC, memory) },
            Instruction::SUB_D => { self.sub(Operand8::RegD, memory) },
            Instruction::SUB_E => { self.sub(Operand8::RegE, memory) },
            Instruction::SUB_H => { self.sub(Operand8::RegH, memory) },
            Instruction::SUB_L => { self.sub(Operand8::RegL, memory) },
            Instruction::SUB_M => { self.sub(Operand8::Memory, memory) },
            Instruction::SUB_A => { self.sub(Operand8::RegA, memory) },
            Instruction::SBB_B => { self.sbb(Operand8::RegB, memory) },
            Instruction::SBB_C => { self.sbb(Operand8::RegC, memory) },
            Instruction::SBB_D => { self.sbb(Operand8::RegD, memory) },
            Instruction::SBB_E => { self.sbb(Operand8::RegE, memory) },
            Instruction::SBB_H => { self.sbb(Operand8::RegH, memory) },
            Instruction::SBB_L => { self.sbb(Operand8::RegL, memory) },
            Instruction::SBB_M => { self.sbb(Operand8::Memory, memory) },
            Instruction::SBB_A => { self.sbb(Operand8::RegA, memory) },
            Instruction::ANA_B => { self.ana(Operand8::RegB, memory) },
            Instruction::ANA_C => { self.ana(Operand8::RegC, memory) },
            Instruction::ANA_D => { self.ana(Operand8::RegD, memory) },
            Instruction::ANA_E => { self.ana(Operand8::RegE, memory) },
            Instruction::ANA_H => { self.ana(Operand8::RegH, memory) },
            Instruction::ANA_L => { self.ana(Operand8::RegL, memory) } ,
            Instruction::ANA_M => { self.ana(Operand8::Memory, memory) },
            Instruction::ANA_A => { self.ana(Operand8::RegA, memory) },
            Instruction::XRA_B => { self.xra(Operand8::RegB, memory) },
            Instruction::XRA_C => { self.xra(Operand8::RegC, memory) },
            Instruction::XRA_D => { self.xra(Operand8::RegD, memory) },
            Instruction::XRA_E => { self.xra(Operand8::RegE, memory) },
            Instruction::XRA_H => { self.xra(Operand8::RegH, memory) },
            Instruction::XRA_L => { self.xra(Operand8::RegL, memory) },
            Instruction::XRA_M => { self.xra(Operand8::Memory, memory) },
            Instruction::XRA_A => { self.xra(Operand8::RegA, memory) },
            Instruction::ORA_B => { self.ora(Operand8::RegB, memory) },
            Instruction::ORA_C => { self.ora(Operand8::RegC, memory) },
            Instruction::ORA_D => { self.ora(Operand8::RegD, memory) },
            Instruction::ORA_E => { self.ora(Operand8::RegE, memory) },
            Instruction::ORA_H => { self.ora(Operand8::RegH, memory) },
            Instruction::ORA_L => { self.ora(Operand8::RegL, memory) },
            Instruction::ORA_M => { self.ora(Operand8::Memory, memory) },
            Instruction::ORA_A => { self.ora(Operand8::RegA, memory) },
            Instruction::CMP_B => { self.cmp(Operand8::RegB, memory) },
            Instruction::CMP_C => { self.cmp(Operand8::RegC, memory) },
            Instruction::CMP_D => { self.cmp(Operand8::RegD, memory) },
            Instruction::CMP_E => { self.cmp(Operand8::RegE, memory) },
            Instruction::CMP_H => { self.cmp(Operand8::RegH, memory) },
            Instruction::CMP_L => { self.cmp(Operand8::RegL, memory) },
            Instruction::CMP_M => { self.cmp(Operand8::Memory, memory) },
            Instruction::CMP_A => { self.cmp(Operand8::RegA, memory) },
            Instruction::RLC => { self.rlc() },
            Instruction::RRC => { self.rrc() },
            Instruction::RAL => { self.ral() },
            Instruction::RAR => { self.rar() },
            Instruction::PUSH_B => { self.push(Operand16::RegPairB, memory) },
            Instruction::PUSH_D => { self.push(Operand16::RegPairD, memory) },
            Instruction::PUSH_H => { self.push(Operand16::RegPairH, memory) },
            Instruction::PUSH_PSW => { self.push(Operand16::PSW, memory) },
            Instruction::POP_B => { self.pop(Operand16::RegPairB, memory) },
            Instruction::POP_D => { self.pop(Operand16::RegPairD, memory) },
            Instruction::POP_H => { self.pop(Operand16::RegPairH, memory) },
            Instruction::POP_PSW => { self.pop(Operand16::PSW, memory) },
            Instruction::DAD_B => { self.dad(Operand16::RegPairB) },
            Instruction::DAD_D => { self.dad(Operand16::RegPairD) },
            Instruction::DAD_H => { self.dad(Operand16::RegPairH) },
            Instruction::DAD_SP => { self.dad(Operand16::SP) },
            Instruction::INX_B => { self.inx(Operand16::RegPairB) },
            Instruction::INX_D => { self.inx(Operand16::RegPairD) },
            Instruction::INX_H => { self.inx(Operand16::RegPairH) },
            Instruction::INX_SP => { self.inx(Operand16::SP) },
            Instruction::DCX_B => { self.dcx(Operand16::RegPairB) },
            Instruction::DCX_D => { self.dcx(Operand16::RegPairD) },
            Instruction::DCX_H => { self.dcx(Operand16::RegPairH) },
            Instruction::DCX_SP => { self.dcx(Operand16::SP) },
            Instruction::XCHG => { self.xchg() },
            Instruction::XTHL => { self.xthl(memory) },
            Instruction::SPHL => { self.sphl() },
            Instruction::LXI_B => { self.lxi(Operand16::RegPairB, memory) },
            Instruction::LXI_D => { self.lxi(Operand16::RegPairD, memory) },
            Instruction::LXI_H => { self.lxi(Operand16::RegPairH, memory) },
            Instruction::LXI_SP => { self.lxi(Operand16::SP, memory) },
            Instruction::MVI_B => { self.mov(Operand8::RegB, Operand8::Immediate, memory) },
            Instruction::MVI_C => { self.mov(Operand8::RegC, Operand8::Immediate, memory) },
            Instruction::MVI_D => { self.mov(Operand8::RegD, Operand8::Immediate, memory) },
            Instruction::MVI_E => { self.mov(Operand8::RegE, Operand8::Immediate, memory) },
            Instruction::MVI_H => { self.mov(Operand8::RegH, Operand8::Immediate, memory) },
            Instruction::MVI_L => { self.mov(Operand8::RegL, Operand8::Immediate, memory) },
            Instruction::MVI_M => { self.mov(Operand8::Memory, Operand8::Immediate, memory) },
            Instruction::MVI_A => { self.mov(Operand8::RegA, Operand8::Immediate, memory) },
            Instruction::ADI => { self.add(Operand8::Immediate, memory) },
            Instruction::ACI => { self.adc(Operand8::Immediate, memory) },
            Instruction::SUI => { self.sub(Operand8::Immediate, memory) },
            Instruction::SBI => { self.sbb(Operand8::Immediate, memory) },
            Instruction::ANI => { self.ana(Operand8::Immediate, memory) },
            Instruction::XRI => { self.xra(Operand8::Immediate, memory) },
            Instruction::ORI => { self.ora(Operand8::Immediate, memory) },
            Instruction::CPI => { self.cmp(Operand8::Immediate, memory) },
            Instruction::STA => { self.sta(memory) },
            Instruction::LDA => { self.lda(memory) },
            Instruction::SHLD => { self.shld(memory) },
            Instruction::LHLD => { self.lhld(memory) },
            Instruction::PCHL => { self.pchl() },
            Instruction::JMP => { self.jmp(ConditionCode::Unconditional, memory) },
            Instruction::JC => { self.jmp(ConditionCode::Carry, memory) },
            Instruction::JNC => { self.jmp(ConditionCode::NoCarry, memory) },
            Instruction::JZ => { self.jmp(ConditionCode::Zero, memory) },
            Instruction::JNZ => { self.jmp(ConditionCode::NotZero, memory) },
            Instruction::JM => { self.jmp(ConditionCode::Minus, memory) },
            Instruction::JP => { self.jmp(ConditionCode::Postive, memory) },
            Instruction::JPE => { self.jmp(ConditionCode::ParityEven, memory) },
            Instruction::JPO => { self.jmp(ConditionCode::ParityOdd, memory) },
            Instruction::CALL => { self.call(ConditionCode::Unconditional, memory) },
            Instruction::CC => { self.call(ConditionCode::Carry, memory) },
            Instruction::CNC => { self.call(ConditionCode::NoCarry, memory) },
            Instruction::CZ => { self.call(ConditionCode::Zero, memory) },
            Instruction::CNZ => { self.call(ConditionCode::NotZero, memory) },
            Instruction::CM => { self.call(ConditionCode::Minus, memory) },
            Instruction::CP => { self.call(ConditionCode::Postive, memory) },
            Instruction::CPE => { self.call(ConditionCode::ParityEven, memory) },
            Instruction::CPO => { self.call(ConditionCode::ParityOdd, memory) },
            Instruction::RET => { self.ret(ConditionCode::Unconditional, memory) },
            Instruction::RC => { self.ret(ConditionCode::Carry, memory) },
            Instruction::RNC => { self.ret(ConditionCode::NoCarry, memory) },
            Instruction::RZ => { self.ret(ConditionCode::Zero, memory) },
            Instruction::RNZ => { self.ret(ConditionCode::NotZero, memory) },
            Instruction::RM => { self.ret(ConditionCode::Minus, memory) },
            Instruction::RP => { self.ret(ConditionCode::Postive, memory) },
            Instruction::RPE => { self.ret(ConditionCode::ParityEven, memory) },
            Instruction::RPO => { self.ret(ConditionCode::ParityOdd, memory) },
            Instruction::RST_1 => { self.rst(0, memory) },
            Instruction::RST_2 => { self.rst(1, memory) },
            Instruction::RST_3 => { self.rst(2, memory) },
            Instruction::RST_4 => { self.rst(3, memory) },
            Instruction::RST_5 => { self.rst(4, memory) },
            Instruction::RST_6 => { self.rst(5, memory) },
            Instruction::RST_7 => { self.rst(6, memory) },
            Instruction::RST_8 => { self.rst(7, memory) },
            Instruction::EI => { self.ei(); 4 },
            Instruction::DI => { self.di(); 4 },
            Instruction::IN => { self.input(memory) },
            Instruction::OUT => { self.output(memory) },
            Instruction::HLT => { self.hlt() },
            _ => {4}
        }
    }

    fn load_imm(&mut self, memory: &impl MemoryAccess) {
        let val = self.fetch_immediate(memory) as u8;
        self.registers.set_z(val);
    }

    fn load_imm16(&mut self, memory: &impl MemoryAccess) {
        let z = self.fetch_immediate(memory);
        self.registers.set_z(z);

        let w = self.fetch_immediate(memory);
        self.registers.set_w(w);
    }

    fn get_src(&mut self, src: Operand8, memory: &impl MemoryAccess) -> u8 {
        match src {
            Operand8::RegB => { self.registers.b() },
            Operand8::RegC => { self.registers.c() },
            Operand8::RegD => { self.registers.d() },
            Operand8::RegE => { self.registers.e() },
            Operand8::RegH => { self.registers.h() },
            Operand8::RegL => { self.registers.l() },
            Operand8::Memory => { memory.read_byte(self.registers.pair_h()) }
            Operand8::RegA => { self.registers.accumulator() },
            Operand8::Immediate => { self.load_imm(memory); self.registers.z() }
        }
    }

    fn write_dst(&mut self, dst: Operand8, val: u8, memory: &mut impl MemoryAccess) {
        match dst {
            Operand8::RegB => { self.registers.set_b(val) },
            Operand8::RegC => { self.registers.set_c(val) },
            Operand8::RegD => { self.registers.set_d(val) },
            Operand8::RegE => { self.registers.set_e(val) },
            Operand8::RegH => { self.registers.set_h(val) },
            Operand8::RegL => { self.registers.set_l(val) },
            Operand8::Memory => { memory.write_byte(self.registers.pair_h(), val); },
            Operand8::RegA => { self.registers.set_accumulator(val) },
            Operand8::Immediate => { panic!("Can't write to immediate!") } 
        }
    }

    fn get_src_16(&mut self, src: Operand16) -> u16 {
        match src {
            Operand16::PSW => { self.registers.psw() },
            Operand16::RegPairB => { self.registers.pair_b() },
            Operand16::RegPairD => { self.registers.pair_d() },
            Operand16::RegPairH => { self.registers.pair_h() },
            Operand16::SP => { self.registers.sp() }
        }
    }

    fn write_dst_16(&mut self, src: Operand16, val: u16) {
        match src {
            Operand16::PSW => { self.registers.set_psw(val) },
            Operand16::RegPairB => { self.registers.set_pair_b(val) },
            Operand16::RegPairD => { self.registers.set_pair_d(val) },
            Operand16::RegPairH => { self.registers.set_pair_h(val) },
            Operand16::SP => { self.registers.set_sp(val) }
        }
    }

    fn set_condition(&mut self, val: u8) {
        self.registers.set_status_zero(val == 0);
        self.registers.set_status_sign(val & 0x80 != 0);
        self.registers.set_status_parity(parity_even(val));
    }

    // Set Carry
    fn stc(&mut self) -> u64 {
        self.registers.set_status_carry(true);
        4
    }

    // Complement Carry
    fn cmc(&mut self) -> u64 {
        self.registers.set_status_carry(!self.registers.status_carry());
        4
    }

    // Increment Register or Memory
    fn inr(&mut self, reg: Operand8, memory: &mut impl MemoryAccess) -> u64 {
        let val = self.get_src(reg, memory);
        let val = val.wrapping_add(1);
        self.write_dst(reg, val, memory);
        self.set_condition(val);
        self.registers.set_status_aux_carry(val & 0x0F == 0x0F);

        if reg == Operand8::Memory {
            return 10
        }
        return 5
    }

    // Decrement Register or Memory
    fn dcr(&mut self, reg: Operand8, memory: &mut impl MemoryAccess) -> u64 {
        // Note: This is equivalent in Intel8080 to adding 0xFF to the value
        // due to the usage of 2s complement representation. Carry bits will be 
        // set accordingly.

        let orig_val = self.get_src(reg, memory);
        let new_val = orig_val.wrapping_add(0xFF);
        self.write_dst(reg, new_val, memory);
        self.set_condition(new_val);
        self.registers.set_status_aux_carry(orig_val & 0xF != 0);

        if reg == Operand8::Memory {
            return 10
        }
        return 5
    }

    // Complement Accumulator
    fn cma(&mut self) -> u64 {
        self.registers.set_accumulator(!self.registers.accumulator());
        4
    }

    // Decimal Adjust Accumulator
    fn daa(&mut self) -> u64 {
        let mut val: u8 = self.registers.accumulator();
        
        let aux_carry = val & 0x0F > 0x09;
        if aux_carry || self.registers.status_aux_carry() {
            val = val.wrapping_add(0x6);
        }

        let carry = val & 0xF0 > 0x90;
        if carry || self.registers.status_carry() {
            val = val.wrapping_add(0x60);
        }

        self.registers.set_accumulator(val);
        self.set_condition(val);
        self.registers.set_status_carry(carry);
        self.registers.set_status_aux_carry(aux_carry);
        4
    }

    // Move
    fn mov(&mut self, dst: Operand8, src: Operand8, memory: &mut impl MemoryAccess) -> u64 {
        let val = self.get_src(src, memory);
        self.write_dst(dst, val, memory);

        if (dst == Operand8::Memory) || (src == Operand8::Memory) || (src == Operand8::Immediate) {
            return 7
        }
        return 5
    }

    // Store Accumulator
    fn stax(&mut self, dst: Operand16, memory: &mut impl MemoryAccess) -> u64 {
        match dst {
            Operand16::RegPairB => {
                memory.write_byte(self.registers.pair_b(), self.registers.accumulator());
            },
            Operand16::RegPairD => {
                memory.write_byte(self.registers.pair_d(), self.registers.accumulator());
            },
            _ => { panic!("Invalid dst passed to STAX: {:?}!", dst) }
        };

        7
    }

    // Load Accumulator
    fn ldax(&mut self, dst: Operand16, memory: &mut impl MemoryAccess) -> u64 {
        match dst {
            Operand16::RegPairB => {
                self.registers.set_accumulator(memory.read_byte(self.registers.pair_b()));
            },
            Operand16::RegPairD => {
                self.registers.set_accumulator(memory.read_byte(self.registers.pair_d()));
            },
            _ => { panic!("Invalid dst passed to LDAX: {:?}!", dst) }
        };

        7
    }

    // ADD Register or Memory to Accumulator
    fn add(&mut self, src: Operand8, memory: &impl MemoryAccess) -> u64 {
        let old_val: u8 = self.registers.accumulator();
        let new_val: u8 = old_val.wrapping_add(self.get_src(src, memory)); 
        
        let carry: bool = check_carry(old_val, new_val);
        let aux_carry: bool = check_aux_carry(old_val, new_val);

        self.registers.set_accumulator(new_val as u8);
        self.set_condition(new_val as u8);
        self.registers.set_status_carry(carry);
        self.registers.set_status_aux_carry(aux_carry);
        
        if (src == Operand8::Memory) || (src == Operand8::Immediate) {
            return 7
        }
        return 4
    }

    // ADD Register or Memory to Accumulator With Carry
    fn adc(&mut self, src: Operand8, memory: &impl MemoryAccess) -> u64 {
        let old_val: u8 = self.registers.accumulator();
        let new_val: u8 = old_val.wrapping_add(self.get_src(src, memory))
            .wrapping_add(self.registers.status_carry() as u8); 
        
        let carry: bool = check_carry(old_val, new_val);
        let aux_carry: bool = check_aux_carry(old_val, new_val);

        self.registers.set_accumulator(new_val);
        self.set_condition(new_val);
        self.registers.set_status_carry(carry);
        self.registers.set_status_aux_carry(aux_carry);
        
        if (src == Operand8::Memory) || (src == Operand8::Immediate) {
            return 7
        }
        return 4
    }

    // Subtract Register or Memory From Accumulator
    fn sub(&mut self, src: Operand8, memory: &impl MemoryAccess) -> u64 {
        let old_val = self.registers.accumulator();
        let new_val = old_val.wrapping_sub(self.get_src(src, memory));

        let carry: bool = !check_carry(old_val, new_val);
        let aux_carry: bool = check_aux_carry(old_val, new_val);

        self.registers.set_accumulator(new_val);
        self.set_condition(new_val);
        self.registers.set_status_carry(carry);
        self.registers.set_status_aux_carry(aux_carry);
        
        if (src == Operand8::Memory) || (src == Operand8::Immediate) {
            return 7
        }
        return 4
    } 

    // Subtract Register or Memory From Accumulator With Borrow
    fn sbb(&mut self, src: Operand8, memory: &impl MemoryAccess) -> u64 {
        let old_val = self.registers.accumulator();
        let new_val = old_val.wrapping_sub(self.get_src(src, memory))
                             .wrapping_sub(self.registers.status_carry() as u8);

        let carry: bool = !check_carry(old_val, new_val);
        let aux_carry: bool = check_aux_carry(old_val, new_val);

        self.registers.set_accumulator(new_val);
        self.set_condition(new_val);
        self.registers.set_status_carry(carry);
        self.registers.set_status_aux_carry(aux_carry);
        
        if (src == Operand8::Memory) || (src == Operand8::Immediate) {
            return 7
        }
        return 4
    }

    // Logical and Register or Memory With Accumulator
    fn ana(&mut self, src: Operand8, memory: &impl MemoryAccess) -> u64 {
        let src = Operand8::from(src);

        let val: u8 = self.registers.accumulator() & self.get_src(src, memory);
        self.set_condition(val);
        self.registers.set_status_carry(false);
        self.registers.set_status_aux_carry(false);
        self.registers.set_accumulator(val);
        
        if (src == Operand8::Memory) || (src == Operand8::Immediate) {
            return 7
        }
        return 4
    }

    // Logical Exclusive-Or Register or Memory with Accumulator
    fn xra(&mut self, src: Operand8, memory: &impl MemoryAccess) -> u64 {
        let val: u8 = self.registers.accumulator() ^ self.get_src(src, memory);
        self.set_condition(val);
        self.registers.set_status_carry(false);
        self.registers.set_status_aux_carry(false);
        self.registers.set_accumulator(val);
        
        if (src == Operand8::Memory) || (src == Operand8::Immediate) {
            return 7
        }
        return 4
    }

    // Logical or Register or Memory with Accumulator
    fn ora(&mut self, src: Operand8, memory: &impl MemoryAccess) -> u64 {
        let val: u8 = self.registers.accumulator() | self.get_src(src, memory);
        self.set_condition(val);
        self.registers.set_status_carry(false);
        self.registers.set_accumulator(val);
        
        if (src == Operand8::Memory) || (src == Operand8::Immediate) {
            return 7
        }
        return 4
    }

    // Compare Register or Memory with Accumulator
    fn cmp(&mut self, src: Operand8, memory: &impl MemoryAccess) -> u64 {
        let old_val = self.registers.accumulator();
        let new_val = old_val.wrapping_add(twos_complement(self.get_src(src, memory)));

        let carry: bool = !check_carry(old_val, new_val);
        let aux_carry: bool = check_aux_carry(old_val, new_val);

        self.set_condition(new_val);
        self.registers.set_status_carry(carry);
        self.registers.set_status_aux_carry(aux_carry);
        
        if (src == Operand8::Memory) || (src == Operand8::Immediate) {
            return 7
        }
        return 4
    }

    // Rotate Accumulator Left
    fn rlc(&mut self) -> u64 {
        let val: u8 = self.registers.accumulator();

        // check left-most bit
        if val & 0x80 != 0 {
            self.registers.set_status_carry(true);        
        }
        else {
            self.registers.set_status_carry(false);
        }

        // shift left and set right-most bit to the previous left-most bit
        let val: u8 = val.rotate_left(1);

        self.registers.set_accumulator(val);
        4
    }

    // Rotate Accumulator Right
    fn rrc(&mut self) -> u64 {
        let val: u8 = self.registers.accumulator();

        // check right-most bit
        if val & 0x01 != 0 {
            self.registers.set_status_carry(true);        
        }
        else {
            self.registers.set_status_carry(false);
        }

        // shift right and set left-most bit to the previous right-most bit
        let val: u8 = val.rotate_right(1);

        self.registers.set_accumulator(val);
        4
    }

    // Rotate Accumulator Left Through Carry
    fn ral(&mut self) -> u64 {
        let val: u8 = self.registers.accumulator();
        let old_carry: bool = self.registers.status_carry();

        // set carry to left-most bit
        self.registers.set_status_carry(val & 0x80 != 0);        

        // shift left and set right-most bit to the previous left-most bit
        let val: u8 = (val << 1) | (if old_carry {1} else {0});

        self.registers.set_accumulator(val);
        4
    }

    // Rotate Accumulator Right Through Carry
    fn rar(&mut self) -> u64 {
        let val: u8 = self.registers.accumulator();
        let old_carry: bool = self.registers.status_carry();

        // set carry to right most bit
        self.registers.set_status_carry(val & 0x01 != 0);        

        // shift right and set left-most bit to the old carry bit value
        let val: u8 = (val >> 1) | (if old_carry {0x80} else {0});

        self.registers.set_accumulator(val);
        4
    }

    // Push Data Onto Stack
    fn push(&mut self, src: Operand16, memory: &mut impl MemoryAccess) -> u64 {
        let first_register = match src {
            Operand16::RegPairB => { self.registers.b() },
            Operand16::RegPairD => { self.registers.d() },
            Operand16::RegPairH => { self.registers.h() },
            Operand16::PSW => { self.registers.status() },
            _ => { panic!("Invalid src passed to PUSH: {:?}!", src) }
        };
        
        let second_register = match src {
            Operand16::RegPairB => { self.registers.c() },
            Operand16::RegPairD => { self.registers.e() },
            Operand16::RegPairH => { self.registers.l() },
            Operand16::PSW => { self.registers.accumulator() },
            _ => { unreachable!() }
        };

        memory.write_byte(self.registers.sp().wrapping_sub(1), first_register);
        memory.write_byte(self.registers.sp().wrapping_sub(2), second_register);
        self.registers.set_sp(self.registers.sp().wrapping_sub(2));

        11
    }

    // Pop Data Off Stack
    fn pop(&mut self, dst: Operand16, memory: &impl MemoryAccess) -> u64 {
        let second_register = memory.read_byte(self.registers.sp());
        let first_register = memory.read_byte(self.registers.sp().wrapping_add(1));

        self.registers.set_sp(self.registers.sp().wrapping_add(2));

        match dst {
            Operand16::RegPairB => {
                self.registers.set_b(first_register);
                self.registers.set_c(second_register);
            },
            Operand16::RegPairD => {
                self.registers.set_d(first_register);
                self.registers.set_e(second_register);
            },
            Operand16::RegPairH => {
                self.registers.set_h(first_register);
                self.registers.set_l(second_register);
            },
            Operand16::PSW => {
                self.registers.set_status(first_register);
                self.registers.set_accumulator(second_register);
            },
            _ => { panic!("Invalid dst passed to POP: {:?}!", dst) }
        };
        10
    }

    // Double Add
    fn dad(&mut self, src: Operand16) -> u64 {
        let val = self.get_src_16(src);
        
        let carry = (val & 0x80) | (self.registers.pair_h() & 0x80) != 0;
        self.registers.set_status_carry(carry);
        
        self.registers.set_pair_h(val + self.registers.pair_h());
        10
    }

    // Increment Register Pair
    fn inx(&mut self, src: Operand16) -> u64 {
        let val = self.get_src_16(src);
        self.write_dst_16(src, val.wrapping_add(1));
        5
    }

    // Decrement Register Pair
    fn dcx(&mut self, src: Operand16) -> u64 {
        let val = self.get_src_16(src);
        self.write_dst_16(src, val.wrapping_sub(1));
        5
    }

    // Exchange Registers
    fn xchg(&mut self) -> u64 {
        let temp = self.registers.pair_h();
        self.registers.set_pair_h(self.registers.pair_d());
        self.registers.set_pair_d(temp);
        4
    }

    // Exchange Stack
    fn xthl(&mut self, memory: &mut impl MemoryAccess) -> u64 {
        let temp = self.registers.h();
        self.registers.set_h(memory.read_byte(self.registers.sp().wrapping_add(1)));
        memory.write_byte(self.registers.sp().wrapping_add(1), temp);
        
        let temp = self.registers.l();
        self.registers.set_l(memory.read_byte(self.registers.sp()));
        memory.write_byte(self.registers.sp(), temp);
        18
    }

    // Load SP From H and L
    fn sphl(&mut self) -> u64 {
        self.registers.set_sp(self.registers.pair_h());
        5
    }

    // Load Register Pair Immediate
    fn lxi(&mut self, dst: Operand16, memory: &impl MemoryAccess) -> u64 {
        self.load_imm16(memory);
        let val = self.registers.pair_w();
        self.write_dst_16(dst, val);
        10
    }

    // Store Accumulator Direct
    fn sta(&mut self, memory: &mut impl MemoryAccess) -> u64 {
        self.load_imm16(memory);
        memory.write_byte(self.registers.pair_w(), self.registers.accumulator());
        13
    }

    // Load Accumulator Direct
    fn lda(&mut self, memory: &mut impl MemoryAccess) -> u64 {
        self.load_imm16(memory);
        self.registers.set_accumulator(memory.read_byte(self.registers.pair_w()));
        13
    }

    // Store H and L Direct
    fn shld(&mut self, memory: &mut impl MemoryAccess) -> u64 {
        self.load_imm16(memory);
        memory.write_byte(self.registers.pair_w(), self.registers.l());
        memory.write_byte(self.registers.pair_w().wrapping_add(1), self.registers.h());
        16
    }

    // Load H and L Direct
    fn lhld(&mut self, memory: &mut impl MemoryAccess) -> u64 {
        self.load_imm16(memory);
        self.registers.set_l(memory.read_byte(self.registers.pair_w()));
        self.registers.set_h(memory.read_byte(self.registers.pair_w().wrapping_add(1)));
        15
    }

    // Load Program Counter
    fn pchl(&mut self) -> u64 {
        self.registers.set_pc(self.registers.pair_h());
        5
    }

    fn check_condition(&self, condition: ConditionCode) -> bool {
        return match condition {
            ConditionCode::NotZero => { !self.registers.status_zero() },
            ConditionCode::Zero => { self.registers.status_zero() },
            ConditionCode::NoCarry => { !self.registers.status_carry() },
            ConditionCode::Carry => { self.registers.status_carry() },
            ConditionCode::ParityOdd => { !self.registers.status_parity() },
            ConditionCode::ParityEven => { self.registers.status_parity() },
            ConditionCode::Postive => { !self.registers.status_sign() },
            ConditionCode::Minus => { self.registers.status_sign() },
            ConditionCode::Unconditional => { true }
        }
    }

    // Jump
    fn jmp(&mut self, condition: ConditionCode, memory: &impl MemoryAccess) -> u64 {
        self.load_imm16(memory);

        if self.check_condition(condition) {
            self.registers.set_pc(self.registers.pair_w());
        }

        return 10
    }

    // Call
    fn call(&mut self, condition: ConditionCode, memory: &mut impl MemoryAccess) -> u64 {
        self.load_imm16(memory);

        if self.check_condition(condition) {
            let hi_addr = (self.registers.pc() >> 8) as u8;
            let lo_addr = (self.registers.pc() & 0xFF) as u8;
            
            memory.write_byte(self.registers.sp(), lo_addr);
            memory.write_byte(self.registers.sp().wrapping_sub(1), hi_addr);
            self.registers.set_sp(self.registers.sp().wrapping_sub(2));
            
            self.registers.set_pc(self.registers.pair_w());
            
            return 17;
        }
        return 11
    }
    
    // Return
    fn ret(&mut self, condition: ConditionCode, memory: &mut impl MemoryAccess) -> u64 {
        if self.check_condition(condition) {
            self.registers.set_sp(self.registers.sp().wrapping_add(2));
            let hi_addr = memory.read_byte(self.registers.sp().wrapping_sub(1));
            let lo_addr = memory.read_byte(self.registers.sp());
            
            let new_pc: u16 = make_u16(hi_addr, lo_addr);
            self.registers.set_pc(new_pc);
            
            return if condition == ConditionCode::Unconditional {10} else {11};
        }
        return 5;
    }
    fn rst(&mut self, exp: u8, memory: &mut impl MemoryAccess) -> u64 {
        let hi_addr = (self.registers.pc() >> 8) as u8;
        let lo_addr = (self.registers.pc() & 0xFF) as u8;
        
        memory.write_byte(self.registers.sp(), lo_addr);
        memory.write_byte(self.registers.sp().wrapping_sub(1), hi_addr);
        self.registers.set_sp(self.registers.sp().wrapping_sub(2));
        
        self.registers.set_pc((exp as u16) << 3);
        11
    }
    // Enable Interrupts
    fn ei(&mut self) -> u64 {
        self.inte = true;
        4
    }

    // Disable Interrupts
    fn di(&mut self) -> u64 {
        self.inte = false;
        4
    }

    // Input
    fn input(&mut self, memory: &impl MemoryAccess) -> u64 {
        self.load_imm(memory);
        self.io_port = self.registers.z();
        self.awaiting_input = true;
        10
    }

    // Output
    fn output(&mut self, memory: &impl MemoryAccess) -> u64 {
        self.load_imm(memory);
        self.io_port = self.registers.z();
        self.output_ready = true;
        10
    }

    // Halt
    fn hlt(&mut self) -> u64 {
        self.stopped = true;
        7
    }
}

// UTILITY FUNCTIONS

fn check_carry(old_val: u8, new_val: u8) -> bool {
    if new_val < old_val {
        return true;
    }
    return false;
}

fn check_aux_carry(old_val: u8, new_val: u8) -> bool {
    if (new_val & 0xF) < (old_val & 0xF) {
        return true;
    }
    return false;
}

fn parity_even(val: u8) -> bool {
    return val.count_ones() % 2 == 0;
}

fn twos_complement(val: u8) -> u8 {
    return (!val).wrapping_add(1)
}

#[inline(always)]
fn make_u16(higher_order: u8, lower_order: u8) -> u16 {
    u16::from_be_bytes([higher_order, lower_order])
}

// END UTILITY FUNCTIONS

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory::Memory as Memory;

    #[test]
    fn test_cma() {
        let mut memory: Memory<1> = Memory::new();
        memory.write_byte(0, Instruction::CMA as u8);

        let mut cpu = Intel8080::new();
        cpu.registers.set_pc(0);
        cpu.registers.set_accumulator(0b01010001);

        cpu.step(&mut memory);

        assert_eq!(cpu.registers.accumulator(), 0b10101110);
    }

    #[test]
    fn test_daa() {
        let mut memory: Memory<1> = Memory::new();
        memory.write_byte(0, Instruction::DAA as u8);

        let mut cpu = Intel8080::new();
        cpu.registers.set_pc(0);
        cpu.registers.set_accumulator(0x9B);
        cpu.registers.set_status_carry(false);
        cpu.registers.set_status_aux_carry(false);

        cpu.step(&mut memory);
        
        assert!(cpu.registers.status_carry());
        assert!(cpu.registers.status_aux_carry());
        assert_eq!(cpu.registers.accumulator(), 0x01);
    }

    #[test]
    fn test_mov() {
        let mut memory: Memory<200> = Memory::new();
        memory.write_byte(0, Instruction::MOV_M_A as u8);
        memory.write_byte(1, Instruction::MOV_B_M as u8);
        memory.write_byte(2, Instruction::MOV_C_B as u8);

        let mut cpu = Intel8080::new();
        cpu.registers.set_pc(0);
        cpu.registers.set_pair_h(160);
        cpu.registers.set_accumulator(0x42);

        cpu.step(&mut memory);
        assert_eq!(0x42, memory.read_byte(160));
        assert_eq!(memory.read_byte(160), cpu.registers.accumulator());

        cpu.step(&mut memory);
        assert_eq!(0x42, cpu.registers.b());
        assert_eq!(memory.read_byte(160), cpu.registers.b());

        cpu.step(&mut memory);
        assert_eq!(0x42, cpu.registers.c());
        assert_eq!(cpu.registers.b(), cpu.registers.c());
    }

    #[test]
    fn test_inr() {
        let mut memory: Memory<1> = Memory::new();
        memory.write_byte(0, Instruction::INR_C as u8);

        let mut cpu = Intel8080::new();
        cpu.registers.set_pc(0);
        cpu.registers.set_c(0x99);

        cpu.step(&mut memory);

        assert_eq!(cpu.registers.c(), 0x9A);
    }
    
    #[test]
    fn test_dcr() {
        let mut memory: Memory<200> = Memory::new();
        memory.write_byte(0, Instruction::DCR_M as u8);
        memory.write_byte(0x50, 0x40);

        let mut cpu = Intel8080::new();
        cpu.registers.set_pc(0);
        cpu.registers.set_h(0x00);
        cpu.registers.set_l(0x50);
        

        cpu.step(&mut memory);

        assert_eq!(memory.read_byte(0x50), 0x3F);
    }

    #[test]
    fn test_stax() {
        let mut memory: Memory<200> = Memory::new();
        memory.write_byte(0, Instruction::STAX_B as u8);
        memory.write_byte(1, Instruction::STAX_D as u8);
        
        let mut cpu = Intel8080::new();
        cpu.registers.set_pc(0);
        cpu.registers.set_accumulator(0x69);
        cpu.registers.set_pair_b(56);
        cpu.registers.set_pair_d(57);

        cpu.step(&mut memory);
        assert_eq!(cpu.registers.accumulator(), memory.read_byte(56));

        cpu.step(&mut memory);
        assert_eq!(cpu.registers.accumulator(), memory.read_byte(57));
    }

    #[test]
    fn test_ldax() {
        let mut memory: Memory<200> = Memory::new();
        memory.write_byte(0, Instruction::LDAX_D as u8);
        memory.write_byte(1, Instruction::LDAX_B as u8);
        memory.write_byte(150, 0x19);
        memory.write_byte(176, 0x35);

        let mut cpu = Intel8080::new();
        cpu.registers.set_pc(0);
        cpu.registers.set_accumulator(0x50);
        cpu.registers.set_pair_d(176);
        cpu.registers.set_pair_b(150);

        cpu.step(&mut memory);
        assert_eq!(cpu.registers.accumulator(), memory.read_byte(176));
        assert_eq!(cpu.registers.accumulator(), 0x35);

        cpu.step(&mut memory);
        assert_eq!(cpu.registers.accumulator(), memory.read_byte(150));
        assert_eq!(cpu.registers.accumulator(), 0x19);
    }

    #[test]
    fn test_sub() {
        let mut memory: Memory<1> = Memory::new();
        memory.write_byte(0, Instruction::SUB_A as u8);

        let mut cpu = Intel8080::new();
        cpu.registers.set_pc(0);
        cpu.registers.set_accumulator(0x3E);

        cpu.step(&mut memory);

        assert_eq!(0, cpu.registers.accumulator());
        assert!(!cpu.registers.status_carry());
        assert!(cpu.registers.status_zero());
        assert!(cpu.registers.status_parity());
        assert!(cpu.registers.status_aux_carry());
        assert!(!cpu.registers.status_sign());
    }

    #[test]
    fn test_sbb() {
        let mut memory: Memory<1> = Memory::new();
        memory.write_byte(0, Instruction::SBB_L as u8);

        let mut cpu = Intel8080::new();
        cpu.registers.set_pc(0);
        cpu.registers.set_accumulator(0x04);
        cpu.registers.set_l(0x02);
        cpu.registers.set_status_carry(true);

        cpu.step(&mut memory);

        assert_eq!(cpu.registers.accumulator(), 0x1);
        assert!(!cpu.registers.status_carry());
        assert!(!cpu.registers.status_zero());
        assert!(!cpu.registers.status_parity());
        assert!(cpu.registers.status_aux_carry());
        assert!(!cpu.registers.status_sign());
    }

    #[test]
    fn test_ana() {
        let mut memory: Memory<1> = Memory::new();
        memory.write_byte(0, Instruction::ANA_C as u8);

        let mut cpu = Intel8080::new();
        cpu.registers.set_pc(0);
        cpu.registers.set_accumulator(0xFC);
        cpu.registers.set_c(0x0F);

        cpu.step(&mut memory);

        assert_eq!(cpu.registers.accumulator(), 0x0C);
        assert!(!cpu.registers.status_carry());
        assert!(!cpu.registers.status_zero());
        assert!(cpu.registers.status_parity());
        assert!(!cpu.registers.status_sign());
    }

    #[test]
    fn test_xra() {
        let mut memory: Memory<2> = Memory::new();
        memory.write_byte(0, Instruction::XRA_A as u8);
        memory.write_byte(1, Instruction::XRA_B as u8);

        let mut cpu = Intel8080::new();
        cpu.registers.set_pc(0);
        cpu.registers.set_accumulator(0xFF);

        cpu.step(&mut memory);

        assert_eq!(cpu.registers.accumulator(), 0);
        assert!(!cpu.registers.status_carry());
        assert!(cpu.registers.status_zero());
        assert!(cpu.registers.status_parity());
        assert!(!cpu.registers.status_aux_carry());
        assert!(!cpu.registers.status_sign());

        cpu.registers.set_accumulator(0xFF);
        cpu.registers.set_b(0x55);

        cpu.step(&mut memory);

        assert_eq!(cpu.registers.accumulator(), 0xAA);
        assert!(!cpu.registers.status_carry());
        assert!(!cpu.registers.status_zero());
        assert!(cpu.registers.status_parity());
        assert!(!cpu.registers.status_aux_carry());
        assert!(cpu.registers.status_sign());
    }

    #[test]
    fn test_ora() {
        let mut memory: Memory<1> = Memory::new();
        memory.write_byte(0, Instruction::ORA_C as u8);

        let mut cpu = Intel8080::new();
        cpu.registers.set_pc(0);
        cpu.registers.set_accumulator(0x33);
        cpu.registers.set_c(0x0F);

        cpu.step(&mut memory);

        assert_eq!(cpu.registers.accumulator(), 0x3F);
        assert!(!cpu.registers.status_carry());
        assert!(!cpu.registers.status_zero());
        assert!(cpu.registers.status_parity());
        assert!(!cpu.registers.status_sign());
    }

    #[test]
    fn test_cmp() {
        let mut memory: Memory<40> = Memory::new();
        memory.write_byte(0, Instruction::CMP_E as u8);
        memory.write_byte(1, Instruction::CMP_H as u8);
        memory.write_byte(2, Instruction::CMP_B as u8);

        let mut cpu = Intel8080::new();
        cpu.registers.set_pc(0);
        cpu.registers.set_accumulator(0x0A);
        cpu.registers.set_e(0x05);

        cpu.step(&mut memory);

        assert_eq!(cpu.registers.accumulator(), 0x0A);
        assert!(!cpu.registers.status_carry());


        cpu.registers.set_accumulator(0x02);
        cpu.registers.set_h(0x05);

        cpu.step(&mut memory);

        assert_eq!(cpu.registers.accumulator(), 0x02);
        assert!(cpu.registers.status_carry());
        assert!(!cpu.registers.status_zero());


        cpu.registers.set_accumulator(0x05);
        cpu.registers.set_b(0x05);

        cpu.step(&mut memory);

        assert_eq!(cpu.registers.accumulator(), 0x05);
        assert!(!cpu.registers.status_carry());
        assert!(cpu.registers.status_zero());
    }

    #[test]
    fn test_rlc() {
        let mut memory: Memory<40> = Memory::new();
        memory.write_byte(0, Instruction::RLC as u8);

        let mut cpu = Intel8080::new();
        cpu.registers.set_pc(0);
        cpu.registers.set_accumulator(0b11110010);

        cpu.step(&mut memory);

        assert_eq!(cpu.registers.accumulator(), 0b11100101);
        assert!(cpu.registers.status_carry());
    }
    
    #[test]
    fn test_rrc() {
        let mut memory: Memory<40> = Memory::new();
        memory.write_byte(0, Instruction::RRC as u8);

        let mut cpu = Intel8080::new();
        cpu.registers.set_pc(0);
        cpu.registers.set_accumulator(0b11110010);

        cpu.step(&mut memory);

        assert_eq!(cpu.registers.accumulator(), 0b01111001);
        assert!(!cpu.registers.status_carry());
    }

    #[test]
    fn test_ral() {
        let mut memory: Memory<40> = Memory::new();
        memory.write_byte(0, Instruction::RAL as u8);

        let mut cpu = Intel8080::new();
        cpu.registers.set_pc(0);
        cpu.registers.set_accumulator(0b10110101);
        cpu.registers.set_status_carry(false);

        cpu.step(&mut memory);

        assert_eq!(cpu.registers.accumulator(), 0b01101010);
        assert!(cpu.registers.status_carry());
    }
    
    #[test]
    fn test_rar() {
        let mut memory: Memory<40> = Memory::new();
        memory.write_byte(0, Instruction::RAR as u8);

        let mut cpu = Intel8080::new();
        cpu.registers.set_pc(0);
        cpu.registers.set_accumulator(0b01101010);
        cpu.registers.set_status_carry(true);

        cpu.step(&mut memory);

        assert_eq!(cpu.registers.accumulator(), 0b10110101);
        assert!(!cpu.registers.status_carry());
    }

    #[test]
    fn test_mvi() {
        let mut memory: Memory<600> = Memory::new();
        memory.write_byte(0, Instruction::MVI_H as u8);
        memory.write_byte(1, 0x01);
        memory.write_byte(2, Instruction::MVI_L as u8);
        memory.write_byte(3, 0xF4);
        memory.write_byte(4, Instruction::MVI_M as u8);
        memory.write_byte(5, 0xFF);

        let mut cpu = Intel8080::new();
        
        cpu.step(&mut memory);

        assert_eq!(cpu.registers.pc(), 0x02);
        assert_eq!(cpu.registers.h(), 0x01);

        cpu.step(&mut memory);
        
        assert_eq!(cpu.registers.pc(), 0x04);
        assert_eq!(cpu.registers.l(), 0xF4);

        cpu.step(&mut memory);

        assert_eq!(cpu.registers.pc(), 0x06);
        assert_eq!(memory.read_byte(0x01F4), 0xFF);
    }

    #[test]
    fn test_adi() {
        let mut memory: Memory<40> = Memory::new();
        memory.write_byte(0, Instruction::ADI as u8);
        memory.write_byte(1, 0x42);

        let mut cpu = Intel8080::new();
        cpu.registers.set_pc(0);
        cpu.registers.set_accumulator(0x14);

        cpu.step(&mut memory);

        assert!(cpu.registers.status_parity());
        assert!(!cpu.registers.status_zero());
        assert!(!cpu.registers.status_sign());
        assert!(!cpu.registers.status_carry());
        assert!(!cpu.registers.status_aux_carry());

        assert_eq!(cpu.registers.accumulator(), 0x56);
    }

    #[test]
    fn test_aci() {
        let mut memory: Memory<2> = Memory::new();
        memory.write_byte(0, Instruction::ACI as u8);
        memory.write_byte(1, 0x42);

        let mut cpu = Intel8080::new();
        cpu.registers.set_pc(0);
        cpu.registers.set_accumulator(0x14);
        cpu.registers.set_status_carry(true);

        cpu.step(&mut memory);

        assert!(!cpu.registers.status_parity());
        assert!(!cpu.registers.status_zero());
        assert!(!cpu.registers.status_sign());
        assert!(!cpu.registers.status_carry());
        assert!(!cpu.registers.status_aux_carry());

        assert_eq!(cpu.registers.accumulator(), 0x57);
    }

    #[test]
    fn test_sui() {
        let mut memory: Memory<2> = Memory::new();
        memory.write_byte(0, Instruction::SUI as u8);
        memory.write_byte(1, 0x01);

        let mut cpu = Intel8080::new();
        cpu.registers.set_pc(0);
        cpu.registers.set_accumulator(0x00);

        cpu.step(&mut memory);

        assert!(cpu.registers.status_parity());
        assert!(!cpu.registers.status_zero());
        assert!(cpu.registers.status_sign());
        assert!(cpu.registers.status_carry());
        assert!(!cpu.registers.status_aux_carry());

        assert_eq!(cpu.registers.accumulator(), 0xFF);
    }

    #[test]
    fn test_sbi() {
        let mut memory: Memory<4> = Memory::new();
        memory.write_byte(0, Instruction::SBI as u8);
        memory.write_byte(1, 0x01);
        
        memory.write_byte(2, Instruction::SBI as u8);
        memory.write_byte(3, 0x01);

        let mut cpu = Intel8080::new();
        cpu.registers.set_pc(0);
        cpu.registers.set_accumulator(0x00);
        cpu.registers.set_status_carry(false);
        
        cpu.step(&mut memory);
        
        assert_eq!(cpu.registers.accumulator(), 0xFF);
        assert_eq!(cpu.registers.pc(), 0x2);
        
        assert!(cpu.registers.status_parity());
        assert!(!cpu.registers.status_zero());
        assert!(cpu.registers.status_sign());
        assert!(cpu.registers.status_carry());
        assert!(!cpu.registers.status_aux_carry());

        cpu.registers.set_accumulator(0x00);
        cpu.step(&mut memory);
        
        assert_eq!(cpu.registers.accumulator(), 0xFE);
        assert_eq!(cpu.registers.pc(), 0x4);
        
        assert!(!cpu.registers.status_parity());
        assert!(!cpu.registers.status_zero());
        assert!(cpu.registers.status_sign());
        assert!(cpu.registers.status_carry());
        assert!(!cpu.registers.status_aux_carry());
    }

    #[test]
    fn test_ani() {
        let mut memory: Memory<2> = Memory::new();
        memory.write_byte(0, Instruction::ANI as u8);
        memory.write_byte(1, 0x0F);

        let mut cpu = Intel8080::new();
        cpu.registers.set_pc(0);
        cpu.registers.set_accumulator(0x3A);

        cpu.step(&mut memory);

        assert!(cpu.registers.status_parity());
        assert!(!cpu.registers.status_zero());
        assert!(!cpu.registers.status_sign());
        assert!(!cpu.registers.status_carry());
        assert!(!cpu.registers.status_aux_carry());

        assert_eq!(cpu.registers.accumulator(), 0x0A);
        assert_eq!(cpu.registers.pc(), 0x02);
    }

    #[test]
    fn test_xri() {
        let mut memory: Memory<2> = Memory::new();
        memory.write_byte(0, Instruction::XRI as u8);
        memory.write_byte(1, 0x81);

        let mut cpu = Intel8080::new();
        cpu.registers.set_pc(0);
        cpu.registers.set_accumulator(0x3B);

        cpu.step(&mut memory);

        assert!(!cpu.registers.status_parity());
        assert!(!cpu.registers.status_zero());
        assert!(cpu.registers.status_sign());
        assert!(!cpu.registers.status_carry());
        assert!(!cpu.registers.status_aux_carry());

        assert_eq!(cpu.registers.accumulator(), 0xBA);
        assert_eq!(cpu.registers.pc(), 0x02);
    }

    #[test]
    fn test_ori() {
        let mut memory: Memory<2> = Memory::new();
        memory.write_byte(0, Instruction::ORI as u8);
        memory.write_byte(1, 0x0F);

        let mut cpu = Intel8080::new();
        cpu.registers.set_pc(0);
        cpu.registers.set_accumulator(0xB5);

        cpu.step(&mut memory);

        assert!(!cpu.registers.status_parity());
        assert!(!cpu.registers.status_zero());
        assert!(cpu.registers.status_sign());
        assert!(!cpu.registers.status_carry());
        assert!(!cpu.registers.status_aux_carry());

        assert_eq!(cpu.registers.accumulator(), 0xBF);
        assert_eq!(cpu.registers.pc(), 0x02);
    }

    #[test]
    fn test_cpi() {
        let mut memory: Memory<2> = Memory::new();
        memory.write_byte(0, Instruction::CPI as u8);
        memory.write_byte(1, 0x40);

        let mut cpu = Intel8080::new();
        cpu.registers.set_pc(0);
        cpu.registers.set_accumulator(0x4A);

        cpu.step(&mut memory);

        assert!(cpu.registers.status_parity());
        assert!(!cpu.registers.status_zero());
        assert!(!cpu.registers.status_sign());
        assert!(!cpu.registers.status_carry());
        assert!(!cpu.registers.status_aux_carry());

        assert_eq!(cpu.registers.accumulator(), 0x4A);
        assert_eq!(cpu.registers.pc(), 0x02);
    }

    #[test]
    fn test_xchg() {
        let mut memory: Memory<1> = Memory::new();
        memory.write_byte(0, Instruction::XCHG as u8);

        let mut cpu = Intel8080::new();
        cpu.registers.set_pc(0);
        cpu.registers.set_d(0x33);
        cpu.registers.set_e(0x55);
        cpu.registers.set_h(0x00);
        cpu.registers.set_l(0xFF);

        cpu.step(&mut memory);

        assert_eq!(cpu.registers.d(), 0x00);
        assert_eq!(cpu.registers.e(), 0xFF);
        assert_eq!(cpu.registers.h(), 0x33);
        assert_eq!(cpu.registers.l(), 0x55);
    }

    #[test]
    fn test_xthl() {
        let mut memory: Memory<40> = Memory::new();
        memory.write_byte(0, Instruction::XTHL as u8);
        memory.write_byte(20, 0xFF);
        memory.write_byte(21, 0xF0);
        memory.write_byte(22, 0x0D);
        memory.write_byte(23, 0xFF);

        let mut cpu = Intel8080::new();
        cpu.registers.set_sp(21);
        cpu.registers.set_pc(0);

        cpu.registers.set_h(0x0B);
        cpu.registers.set_l(0x3C);

        let initial_status = cpu.registers.status();
        cpu.step(&mut memory);

        assert_eq!(cpu.registers.h(), 0x0D);
        assert_eq!(cpu.registers.l(), 0xF0);
        assert_eq!(initial_status, cpu.registers.status());

        assert_eq!(memory.read_byte(20), 0xFF);
        assert_eq!(memory.read_byte(21), 0x3C);
        assert_eq!(memory.read_byte(22), 0x0B);
        assert_eq!(memory.read_byte(23), 0xFF);
    }

    #[test]
    fn test_sphl() {
        let mut memory: Memory<1> = Memory::new();
        memory.write_byte(0, Instruction::SPHL as u8);

        let mut cpu = Intel8080::new();
        cpu.registers.set_pc(0);
        cpu.registers.set_h(0x50);
        cpu.registers.set_l(0x6C);

        cpu.step(&mut memory);

        assert_eq!(cpu.registers.sp(), 0x506C)
    }

    #[test]
    fn test_sta_lda() {
        let mut memory: Memory<500> = Memory::new();
        memory.write_byte(0, Instruction::STA as u8);
        memory.write_byte(1, 0x24); // lo addr
        memory.write_byte(2, 0x01); // hi addr
        memory.write_byte(3, Instruction::LDA as u8);
        memory.write_byte(4, 0x56); // lo addr
        memory.write_byte(5, 0x01); // hi addr
        memory.write_byte(0x0156, 0x72);

        let mut cpu = Intel8080::new();
        cpu.registers.set_pc(0);
        cpu.registers.set_accumulator(0x34);

        cpu.step(&mut memory);

        assert_eq!(memory.read_byte(0x0124), 0x34);
        assert_eq!(cpu.registers.pc(), 0x3);

        cpu.step(&mut memory);

        assert_eq!(cpu.registers.accumulator(), 0x72);
        assert_eq!(cpu.registers.pc(), 0x6);
    }

    #[test]
    fn test_shld_lhld() {
        let mut memory: Memory<650> = Memory::new();
        memory.write_byte(0, Instruction::SHLD as u8);
        memory.write_byte(1, 0x0A); // lo addr
        memory.write_byte(2, 0x01); // hi addr
        memory.write_byte(3, Instruction::LHLD as u8);
        memory.write_byte(4, 0x5B); // lo addr
        memory.write_byte(5, 0x02); // hi addr
        memory.write_byte(0x025B, 0xFF);
        memory.write_byte(0x025C, 0x03);

        let mut cpu = Intel8080::new();
        cpu.registers.set_pc(0);
        cpu.registers.set_h(0xAE);
        cpu.registers.set_l(0x29);

        cpu.step(&mut memory);

        assert_eq!(memory.read_byte(0x010A), 0x29);
        assert_eq!(memory.read_byte(0x010B), 0xAE);
        assert_eq!(cpu.registers.pc(), 0x3);

        cpu.step(&mut memory);
        assert_eq!(cpu.registers.h(), 0x03);
        assert_eq!(cpu.registers.l(), 0xFF);
        assert_eq!(cpu.registers.pc(), 0x6);
    }

    #[test]
    fn test_dad() {
        let mut memory: Memory<1> = Memory::new();
        memory.write_byte(0, Instruction::DAD_B as u8);

        let mut cpu = Intel8080::new();
        cpu.registers.set_pc(0);
        cpu.registers.set_b(0x33);
        cpu.registers.set_c(0x9F);
        cpu.registers.set_h(0xA1);
        cpu.registers.set_l(0x7B);

        cpu.step(&mut memory);

        assert_eq!(cpu.registers.pair_h(), 0xD51A);
    }

    #[test]
    fn test_dcx() {
        let mut memory: Memory<1> = Memory::new();
        memory.write_byte(0, Instruction::DCX_H as u8);

        let mut cpu = Intel8080::new();
        cpu.registers.set_h(0x98);
        cpu.registers.set_l(0x00);

        cpu.step(&mut memory);

        assert_eq!(cpu.registers.h(), 0x97);
        assert_eq!(cpu.registers.l(), 0xFF);
    }

    #[test]
    fn test_inx() {
        let mut memory: Memory<1> = Memory::new();
        memory.write_byte(0, Instruction::INX_D as u8);

        let mut cpu = Intel8080::new();
        cpu.registers.set_d(0x38);
        cpu.registers.set_e(0xFF);

        cpu.step(&mut memory);

        assert_eq!(cpu.registers.d(), 0x39);
        assert_eq!(cpu.registers.e(), 0x00);
    }

    #[test]
    fn test_push_pop() {
        let mut memory: Memory<100> = Memory::new();
        memory.write_byte(0, Instruction::PUSH_D as u8);
        memory.write_byte(1, Instruction::POP_H as u8);

        let mut cpu = Intel8080::new();
        cpu.registers.set_d(0xBF);
        cpu.registers.set_e(0x9D);
        cpu.registers.set_sp(86);

        cpu.step(&mut memory);

        assert_eq!(cpu.registers.sp(), 84);
        assert_eq!(memory.read_byte(85), 0xBF);
        assert_eq!(memory.read_byte(84), 0x9D);

        cpu.step(&mut memory);

        assert_eq!(cpu.registers.sp(), 86);
        assert_eq!(cpu.registers.l(), 0x9D);
        assert_eq!(cpu.registers.h(), 0xBF);
    }

    #[test]
    fn test_pchl() {
        let mut memory: Memory<1> = Memory::new();
        memory.write_byte(0, Instruction::PCHL as u8);

        let mut cpu = Intel8080::new();
        cpu.registers.set_pc(0);
        cpu.registers.set_h(0x41);
        cpu.registers.set_l(0x3E);

        cpu.step(&mut memory);

        assert_eq!(cpu.registers.pc(), 0x413E);
    }

    #[test]
    fn test_jmp() {
        let mut memory: Memory<0x120> = Memory::new();
        memory.write_byte(0x10, Instruction::JMP as u8);
        memory.write_byte(0x11, 0x02); // lo addr
        memory.write_byte(0x12, 0x01); // hi addr
        memory.write_byte(0x102, Instruction::JC as u8);

        let mut cpu = Intel8080::new();
        cpu.registers.set_pc(0x10);

        cpu.step(&mut memory);

        assert_eq!(cpu.registers.pc(), 0x0102);

        
        cpu.registers.set_status_carry(false);

        cpu.step(&mut memory);

        assert_eq!(cpu.registers.pc(), 0x0105);
    }

    #[test]
    fn test_call() {
        let mut memory: Memory<20> = Memory::new();
        memory.write_byte(0, Instruction::CALL as u8);
        memory.write_byte(1, 5); // lo addr
        memory.write_byte(2, 0); // hi addr 
        
        memory.write_byte(7, 0xFF);
        memory.write_byte(8, 0xFF);
        memory.write_byte(9, 0xFF);
        memory.write_byte(10, 0xFF);
        memory.write_byte(11, 0xFF);

        let mut cpu = Intel8080::new();

        cpu.registers.set_pc(0);
        cpu.registers.set_sp(10);

        cpu.step(&mut memory);

        assert_eq!(cpu.registers.pc(), 0x05);
        assert_eq!(cpu.registers.sp(), 0x08);
        assert_eq!(memory.read_byte( 7), 0xFF);
        assert_eq!(memory.read_byte( 8), 0xFF);
        assert_eq!(memory.read_byte( 9), 0x00);
        assert_eq!(memory.read_byte(10), 0x03);
        assert_eq!(memory.read_byte(11), 0xFF);
    }

    #[test]
    fn test_ret() {
        let mut memory: Memory<20> = Memory::new();
        memory.write_byte( 0, Instruction::RET as u8);
        memory.write_byte( 9, 0x00);
        memory.write_byte(10, 0x06);

        let mut cpu = Intel8080::new();
        cpu.registers.set_pc(0x00);
        cpu.registers.set_sp(0x08);

        cpu.step(&mut memory);

        assert_eq!(cpu.registers.pc(), 0x06);
        assert_eq!(cpu.registers.sp(), 0x0A);
    }

    #[test] 
    fn test_rst() {
        let mut memory: Memory<100> = Memory::new();
        memory.write_byte(0, Instruction::RST_2 as u8);

        let mut cpu = Intel8080::new();
        cpu.registers.set_pc(0x00);
        cpu.registers.set_sp(0x10);

        let cycles = cpu.step(&mut memory);
        
        assert_eq!(cycles, 11);
        assert_eq!(cpu.registers.sp(), 0x0E);
        assert_eq!(cpu.registers.pc(), 0x08);
        assert_eq!(memory.read_byte(0x0F), 0x00);
        assert_eq!(memory.read_byte(0x10), 0x01);
    }

    #[test]
    fn test_input() {
        let mut memory: Memory<3> = Memory::new();
        memory.write_byte(0, Instruction::IN as u8);
        memory.write_byte(1, 0x52);
        memory.write_byte(2, Instruction::NOP as u8);

        let mut cpu = Intel8080::new();

        cpu.step(&mut memory);
        
        assert!(cpu.awaiting_input());
        assert_eq!(cpu.active_io_port(), 0x52);

        cpu.write_input(0xFF);

        cpu.step(&mut memory);
        assert!(!cpu.awaiting_input());
        assert_eq!(cpu.registers.accumulator(), 0xFF);
    }

    #[test]
    fn test_output() {
        let mut memory: Memory<3> = Memory::new();
        memory.write_byte(0, Instruction::OUT as u8);
        memory.write_byte(1, 0x3C);
        memory.write_byte(2, Instruction::NOP as u8);

        let mut cpu = Intel8080::new();
        cpu.registers.set_accumulator(0x5A);

        cpu.step(&mut memory);
        
        assert!(cpu.output_ready());
        assert_eq!(cpu.active_io_port(), 0x3C);
        assert_eq!(cpu.read_output(), 0x5A);

        cpu.step(&mut memory);
        assert!(!cpu.output_ready());

    }

    #[test]
    fn test_hlt() {
        let mut memory: Memory<1> = Memory::new();
        memory.write_byte(0, Instruction::HLT as u8);

        let mut cpu = Intel8080::new();

        let cycles = cpu.step(&mut memory);

        assert_eq!(cycles, 7);
        assert!(cpu.stopped);
        assert_eq!(cpu.step(&mut memory), 0);
        assert_eq!(cpu.registers.pc(), 0x01);
    }

    #[test]
    fn test_interrupts() {
        let mut memory: Memory<4> = Memory::new();
        memory.write_byte(0, Instruction::EI as u8);
        memory.write_byte(1, Instruction::MOV_A_L as u8);
        memory.write_byte(2, Instruction::DI as u8);
        memory.write_byte(3, Instruction::XRA_A as u8);
        
        let mut cpu = Intel8080::new();
        cpu.registers.set_accumulator(0x75);
        cpu.registers.set_l(0x85);
        cpu.registers.set_h(0x55);

        cpu.step(&mut memory); // EI

        cpu.interrupt(Instruction::MOV_A_H);

        cpu.step(&mut memory); // interrupt
        assert_eq!(cpu.registers.pc(), 0x01);
        assert_eq!(cpu.registers.accumulator(), cpu.registers.h());
        assert_ne!(cpu.registers.accumulator(), cpu.registers.l());

        cpu.step(&mut memory); // MOV_A_L
        cpu.step(&mut memory); // DI
        
        cpu.interrupt(Instruction::MOV_H_A);

        cpu.step(&mut memory); // XRA_A
        assert_eq!(cpu.registers.pc(), 0x04);
        assert_ne!(cpu.registers.h(), cpu.registers.accumulator());
        assert_eq!(cpu.registers.accumulator(), 0x00);
    }
}
