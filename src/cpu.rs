use std::usize;

use crate::registers::Registers as Registers;
use crate::utils::*;

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
        match orig {
            0x0 => return Instruction::NOP,
            0x37 => return Instruction::STC,
            0x3f => return Instruction::CMC,
            0x4 => return Instruction::INR_B,
            0xc => return Instruction::INR_C,
            0x14 => return Instruction::INR_D,
            0x1c => return Instruction::INR_E,
            0x24 => return Instruction::INR_H,
            0x2c => return Instruction::INR_L,
            0x34 => return Instruction::INR_M,
            0x3c => return Instruction::INR_A,
            0x5 => return Instruction::DCR_B,
            0xd => return Instruction::DCR_C,
            0x15 => return Instruction::DCR_D,
            0x1d => return Instruction::DCR_E,
            0x25 => return Instruction::DCR_H,
            0x2d => return Instruction::DCR_L,
            0x35 => return Instruction::DCR_M,
            0x3d => return Instruction::DCR_A,
            0x2f => return Instruction::CMA,
            0x27 => return Instruction::DAA,
            0x40 => return Instruction::MOV_B_B,
            0x41 => return Instruction::MOV_B_C,
            0x42 => return Instruction::MOV_B_D,
            0x43 => return Instruction::MOV_B_E,
            0x44 => return Instruction::MOV_B_H,
            0x45 => return Instruction::MOV_B_L,
            0x46 => return Instruction::MOV_B_M,
            0x47 => return Instruction::MOV_B_A,
            0x48 => return Instruction::MOV_C_B,
            0x49 => return Instruction::MOV_C_C,
            0x4a => return Instruction::MOV_C_D,
            0x4b => return Instruction::MOV_C_E,
            0x4c => return Instruction::MOV_C_H,
            0x4d => return Instruction::MOV_C_L,
            0x4e => return Instruction::MOV_C_M,
            0x4f => return Instruction::MOV_C_A,
            0x50 => return Instruction::MOV_D_B,
            0x51 => return Instruction::MOV_D_C,
            0x52 => return Instruction::MOV_D_D,
            0x53 => return Instruction::MOV_D_E,
            0x54 => return Instruction::MOV_D_H,
            0x55 => return Instruction::MOV_D_L,
            0x56 => return Instruction::MOV_D_M,
            0x57 => return Instruction::MOV_D_A,
            0x58 => return Instruction::MOV_E_B,
            0x59 => return Instruction::MOV_E_C,
            0x5a => return Instruction::MOV_E_D,
            0x5b => return Instruction::MOV_E_E,
            0x5c => return Instruction::MOV_E_H,
            0x5d => return Instruction::MOV_E_L,
            0x5e => return Instruction::MOV_E_M,
            0x5f => return Instruction::MOV_E_A,
            0x60 => return Instruction::MOV_H_B,
            0x61 => return Instruction::MOV_H_C,
            0x62 => return Instruction::MOV_H_D,
            0x63 => return Instruction::MOV_H_E,
            0x64 => return Instruction::MOV_H_H,
            0x65 => return Instruction::MOV_H_L,
            0x66 => return Instruction::MOV_H_M,
            0x67 => return Instruction::MOV_H_A,
            0x68 => return Instruction::MOV_L_B,
            0x69 => return Instruction::MOV_L_C,
            0x6a => return Instruction::MOV_L_D,
            0x6b => return Instruction::MOV_L_E,
            0x6c => return Instruction::MOV_L_H,
            0x6d => return Instruction::MOV_L_L,
            0x6e => return Instruction::MOV_L_M,
            0x6f => return Instruction::MOV_L_A,
            0x70 => return Instruction::MOV_M_B,
            0x71 => return Instruction::MOV_M_C,
            0x72 => return Instruction::MOV_M_D,
            0x73 => return Instruction::MOV_M_E,
            0x74 => return Instruction::MOV_M_H,
            0x75 => return Instruction::MOV_M_L,
            0x77 => return Instruction::MOV_M_A,
            0x78 => return Instruction::MOV_A_B,
            0x79 => return Instruction::MOV_A_C,
            0x7a => return Instruction::MOV_A_D,
            0x7b => return Instruction::MOV_A_E,
            0x7c => return Instruction::MOV_A_H,
            0x7d => return Instruction::MOV_A_L,
            0x7e => return Instruction::MOV_A_M,
            0x7f => return Instruction::MOV_A_A,
            0x2 => return Instruction::STAX_B,
            0x12 => return Instruction::STAX_D,
            0xa => return Instruction::LDAX_B,
            0x1a => return Instruction::LDAX_D,
            0x80 => return Instruction::ADD_B,
            0x81 => return Instruction::ADD_C,
            0x82 => return Instruction::ADD_D,
            0x83 => return Instruction::ADD_E,
            0x84 => return Instruction::ADD_H,
            0x85 => return Instruction::ADD_L,
            0x86 => return Instruction::ADD_M,
            0x87 => return Instruction::ADD_A,
            0x88 => return Instruction::ADC_B,
            0x89 => return Instruction::ADC_C,
            0x8a => return Instruction::ADC_D,
            0x8b => return Instruction::ADC_E,
            0x8c => return Instruction::ADC_H,
            0x8d => return Instruction::ADC_L,
            0x8e => return Instruction::ADC_M,
            0x8f => return Instruction::ADC_A,
            0x90 => return Instruction::SUB_B,
            0x91 => return Instruction::SUB_C,
            0x92 => return Instruction::SUB_D,
            0x93 => return Instruction::SUB_E,
            0x94 => return Instruction::SUB_H,
            0x95 => return Instruction::SUB_L,
            0x96 => return Instruction::SUB_M,
            0x97 => return Instruction::SUB_A,
            0x98 => return Instruction::SBB_B,
            0x99 => return Instruction::SBB_C,
            0x9a => return Instruction::SBB_D,
            0x9b => return Instruction::SBB_E,
            0x9c => return Instruction::SBB_H,
            0x9d => return Instruction::SBB_L,
            0x9e => return Instruction::SBB_M,
            0x9f => return Instruction::SBB_A,
            0xa0 => return Instruction::ANA_B,
            0xa1 => return Instruction::ANA_C,
            0xa2 => return Instruction::ANA_D,
            0xa3 => return Instruction::ANA_E,
            0xa4 => return Instruction::ANA_H,
            0xa5 => return Instruction::ANA_L,
            0xa6 => return Instruction::ANA_M,
            0xa7 => return Instruction::ANA_A,
            0xa8 => return Instruction::XRA_B,
            0xa9 => return Instruction::XRA_C,
            0xaa => return Instruction::XRA_D,
            0xab => return Instruction::XRA_E,
            0xac => return Instruction::XRA_H,
            0xad => return Instruction::XRA_L,
            0xae => return Instruction::XRA_M,
            0xaf => return Instruction::XRA_A,
            0xb0 => return Instruction::ORA_B,
            0xb1 => return Instruction::ORA_C,
            0xb2 => return Instruction::ORA_D,
            0xb3 => return Instruction::ORA_E,
            0xb4 => return Instruction::ORA_H,
            0xb5 => return Instruction::ORA_L,
            0xb6 => return Instruction::ORA_M,
            0xb7 => return Instruction::ORA_A,
            0xb8 => return Instruction::CMP_B,
            0xb9 => return Instruction::CMP_C,
            0xba => return Instruction::CMP_D,
            0xbb => return Instruction::CMP_E,
            0xbc => return Instruction::CMP_H,
            0xbd => return Instruction::CMP_L,
            0xbe => return Instruction::CMP_M,
            0xbf => return Instruction::CMP_A,
            0x7 => return Instruction::RLC,
            0xf => return Instruction::RRC,
            0x17 => return Instruction::RAL,
            0x1f => return Instruction::RAR,
            0xc5 => return Instruction::PUSH_B,
            0xd5 => return Instruction::PUSH_D,
            0xe5 => return Instruction::PUSH_H,
            0xf5 => return Instruction::PUSH_PSW,
            0xc1 => return Instruction::POP_B,
            0xd1 => return Instruction::POP_D,
            0xe1 => return Instruction::POP_H,
            0xf1 => return Instruction::POP_PSW,
            0x9 => return Instruction::DAD_B,
            0x19 => return Instruction::DAD_D,
            0x29 => return Instruction::DAD_H,
            0x39 => return Instruction::DAD_SP,
            0x3 => return Instruction::INX_B,
            0x13 => return Instruction::INX_D,
            0x23 => return Instruction::INX_H,
            0x33 => return Instruction::INX_SP,
            0xb => return Instruction::DCX_B,
            0x1b => return Instruction::DCX_D,
            0x2b => return Instruction::DCX_H,
            0x3b => return Instruction::DCX_SP,
            0xeb => return Instruction::XCHG,
            0xe3 => return Instruction::XTHL,
            0xf9 => return Instruction::SPHL,
            0x1 => return Instruction::LXI_B,
            0x11 => return Instruction::LXI_D,
            0x21 => return Instruction::LXI_H,
            0x31 => return Instruction::LXI_SP,
            0x6 => return Instruction::MVI_B,
            0xe => return Instruction::MVI_C,
            0x16 => return Instruction::MVI_D,
            0x1e => return Instruction::MVI_E,
            0x26 => return Instruction::MVI_H,
            0x2e => return Instruction::MVI_L,
            0x36 => return Instruction::MVI_M,
            0x3e => return Instruction::MVI_A,
            0xc6 => return Instruction::ADI,
            0xce => return Instruction::ACI,
            0xd6 => return Instruction::SUI,
            0xde => return Instruction::SBI,
            0xe6 => return Instruction::ANI,
            0xee => return Instruction::XRI,
            0xf6 => return Instruction::ORI,
            0xfe => return Instruction::CPI,
            0x32 => return Instruction::STA,
            0x3a => return Instruction::LDA,
            0x22 => return Instruction::SHLD,
            0x2a => return Instruction::LHLD,
            0xe9 => return Instruction::PCHL,
            0xc3 => return Instruction::JMP,
            0xda => return Instruction::JC,
            0xd2 => return Instruction::JNC,
            0xca => return Instruction::JZ,
            0xc2 => return Instruction::JNZ,
            0xfa => return Instruction::JM,
            0xf2 => return Instruction::JP,
            0xea => return Instruction::JPE,
            0xe2 => return Instruction::JPO,
            0xcd => return Instruction::CALL,
            0xdc => return Instruction::CC,
            0xd4 => return Instruction::CNC,
            0xcc => return Instruction::CZ,
            0xc4 => return Instruction::CNZ,
            0xfc => return Instruction::CM,
            0xf4 => return Instruction::CP,
            0xec => return Instruction::CPE,
            0xe4 => return Instruction::CPO,
            0xc9 => return Instruction::RET,
            0xd8 => return Instruction::RC,
            0xd0 => return Instruction::RNC,
            0xc8 => return Instruction::RZ,
            0xc0 => return Instruction::RNZ,
            0xf8 => return Instruction::RM,
            0xf0 => return Instruction::RP,
            0xe8 => return Instruction::RPE,
            0xe0 => return Instruction::RPO,
            0xc7 => return Instruction::RST_1,
            0xcf => return Instruction::RST_2,
            0xd7 => return Instruction::RST_3,
            0xdf => return Instruction::RST_4,
            0xe7 => return Instruction::RST_5,
            0xef => return Instruction::RST_6,
            0xf7 => return Instruction::RST_7,
            0xff => return Instruction::RST_8,
            0xfb => return Instruction::EI,
            0xf3 => return Instruction::DI,
            0xdb => return Instruction::IN,
            0xd3 => return Instruction::OUT,
            0x76 => return Instruction::HLT,
            _ => return Instruction::NOP
        }
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

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
enum Operand16 {
    RegPairB = 0,
    RegPairD = 1,
    RegPairH = 2,
    PSW = 3,
    Immediate = 4,
    SP = 5
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
enum MachineCycle {
    InstructionFetch,
    MemoryRead,
    MemoryWrite,
    StackRead,
    StackWrite,
    InputRead,
    OutputWrite,
    InterruptAcknowledge,
    HaltAcknowledge,
    InterruptAcknowledgeWhileHalt
}


pub struct Intel8080<'a, const N: usize> {
    registers: Registers,
    memory: &'a mut [u8; N],
    status_callback: Option<fn(u8)>,
    stopped: bool,
    inte: bool
}

impl<'a, const N: usize> Intel8080<'a, N> {
    pub fn new(memory: &'a mut [u8; N]) -> Self {
        Self {
            registers: Registers::new(),
            memory,
            status_callback: None,
            stopped: false,
            inte: false
        }
    }
    
    pub fn step(&mut self) {
        let instruction = self.fetch_instruction();
        self.do_instruction(instruction)
    }

    pub fn interrupt(&mut self, instruction: Instruction) {
        self.do_instruction(instruction)
    }

    pub fn set_machine_cycle_callback(&mut self, func: Option<fn(u8)>) {
        self.status_callback = func
    }

    fn start_machine_cycle(&self, cycle_type: MachineCycle) {
        if self.status_callback.is_some() { 
            let status: u8 = match cycle_type {
                MachineCycle::InstructionFetch => { 0b10100010 },
                MachineCycle::MemoryRead => { 0b10000010},
                MachineCycle::MemoryWrite => { 0b00000000 },
                MachineCycle::StackRead => { 0b10000110 },
                MachineCycle::StackWrite => { 0b00000100},
                MachineCycle::InputRead => { 0b01000010 },
                MachineCycle::OutputWrite => { 0b00010000 },
                MachineCycle::InterruptAcknowledge => { 0b00100011 },
                MachineCycle::HaltAcknowledge => { 0b10001010 },
                MachineCycle::InterruptAcknowledgeWhileHalt => { 0b00101011 }
            };

            self.status_callback.unwrap()(status);
        }
    }

    fn fetch_instruction(&mut self) -> Instruction {
        self.start_machine_cycle(MachineCycle::InstructionFetch);
        let instruction: Instruction = Instruction::from(self.memory[self.registers.pc() as usize]);   
        self.registers.set_pc(self.registers.pc() + 1);
        return instruction
    }

    fn read_memory(&self, addr: u16) -> u8 {
        self.start_machine_cycle(MachineCycle::MemoryRead);
        self.memory[addr as usize]
    }

    fn write_memory(&mut self, addr: u16, val: u8) {
        self.start_machine_cycle(MachineCycle::MemoryWrite);
        self.memory[addr as usize] = val
    }

    fn pop_stack(&mut self) -> u8 {
        self.start_machine_cycle(MachineCycle::StackRead);
        self.registers.set_sp(self.registers.sp() + 1);
        self.memory[self.registers.sp() as usize - 1]
    }

    fn push_stack(&mut self, val: u8) {
        self.start_machine_cycle(MachineCycle::StackWrite);
        self.registers.set_sp(self.registers.sp() - 1);
        self.memory[self.registers.sp() as usize - 1] = val;
    }


    fn do_instruction(&mut self, instruction: Instruction) -> usize {
        return match instruction {
            Instruction::STC => { self.stc(); 4 },
            Instruction::CMC => { self.cmc(); 4 },
            Instruction::INR_B => { self.inr(Operand8::RegB); 5 },
            Instruction::INR_C => { self.inr(Operand8::RegC); 5 },
            Instruction::INR_D => { self.inr(Operand8::RegD); 5 },
            Instruction::INR_E => { self.inr(Operand8::RegE); 5 },
            Instruction::INR_H => { self.inr(Operand8::RegH); 5 },
            Instruction::INR_L => { self.inr(Operand8::RegL); 5 },
            Instruction::INR_M => { self.inr(Operand8::Memory); 10 },
            Instruction::INR_A => { self.inr(Operand8::RegA); 5 },
            Instruction::DCR_B => { self.dcr(Operand8::RegB); 5 },
            Instruction::DCR_C => { self.dcr(Operand8::RegC); 5  },
            Instruction::DCR_D => { self.dcr(Operand8::RegD); 5 },
            Instruction::DCR_E => { self.dcr(Operand8::RegE); 5 },
            Instruction::DCR_H => { self.dcr(Operand8::RegH); 5 },
            Instruction::DCR_L => { self.dcr(Operand8::RegL); 5 },
            Instruction::DCR_M => { self.dcr(Operand8::Memory); 10 },
            Instruction::DCR_A => { self.dcr(Operand8::RegA); 5 },
            Instruction::CMA => { self.cma(); 4 },
            Instruction::DAA => { self.daa(); 4 },
            Instruction::MOV_B_C => { self.mov(Operand8::RegB, Operand8::RegC); 5; 5 },
            Instruction::MOV_B_D => { self.mov(Operand8::RegB, Operand8::RegD); 5; 5 },
            Instruction::MOV_B_E => { self.mov(Operand8::RegB, Operand8::RegE); 5; 5 },
            Instruction::MOV_B_H => { self.mov(Operand8::RegB, Operand8::RegH); 5; 5 },
            Instruction::MOV_B_L => { self.mov(Operand8::RegB, Operand8::RegL); 5; 5 },
            Instruction::MOV_B_M => { self.mov(Operand8::RegB, Operand8::Memory); 7 },
            Instruction::MOV_B_A => { self.mov(Operand8::RegB, Operand8::RegA); 5 },
            Instruction::MOV_C_B => { self.mov(Operand8::RegC, Operand8::RegB); 5 },
            Instruction::MOV_C_D => { self.mov(Operand8::RegC, Operand8::RegD); 5 },
            Instruction::MOV_C_E => { self.mov(Operand8::RegC, Operand8::RegE); 5 },
            Instruction::MOV_C_H => { self.mov(Operand8::RegC, Operand8::RegH); 5 },
            Instruction::MOV_C_L => { self.mov(Operand8::RegC, Operand8::RegL); 5 },
            Instruction::MOV_C_M => { self.mov(Operand8::RegC, Operand8::Memory); 7 },
            Instruction::MOV_C_A => { self.mov(Operand8::RegC, Operand8::RegA); 5 },
            Instruction::MOV_D_B => { self.mov(Operand8::RegD, Operand8::RegB); 5 },
            Instruction::MOV_D_C => { self.mov(Operand8::RegD, Operand8::RegC); 5 },
            Instruction::MOV_D_E => { self.mov(Operand8::RegD, Operand8::RegE); 5 },
            Instruction::MOV_D_H => { self.mov(Operand8::RegD, Operand8::RegH); 5 },
            Instruction::MOV_D_L => { self.mov(Operand8::RegD, Operand8::RegL); 5 },
            Instruction::MOV_D_M => { self.mov(Operand8::RegD, Operand8::Memory); 7 },
            Instruction::MOV_D_A => { self.mov(Operand8::RegD, Operand8::RegA); 5 },
            Instruction::MOV_E_B => { self.mov(Operand8::RegE, Operand8::RegB); 5 },
            Instruction::MOV_E_C => { self.mov(Operand8::RegE, Operand8::RegC); 5 },
            Instruction::MOV_E_D => { self.mov(Operand8::RegE, Operand8::RegD); 5 },
            Instruction::MOV_E_H => { self.mov(Operand8::RegE, Operand8::RegH); 5 },
            Instruction::MOV_E_L => { self.mov(Operand8::RegE, Operand8::RegL); 5 },
            Instruction::MOV_E_M => { self.mov(Operand8::RegE, Operand8::Memory); 7 },
            Instruction::MOV_E_A => { self.mov(Operand8::RegE, Operand8::RegA); 5 },
            Instruction::MOV_H_B => { self.mov(Operand8::RegH, Operand8::RegB); 5 },
            Instruction::MOV_H_C => { self.mov(Operand8::RegH, Operand8::RegC); 5 },
            Instruction::MOV_H_D => { self.mov(Operand8::RegH, Operand8::RegD); 5 },
            Instruction::MOV_H_E => { self.mov(Operand8::RegE, Operand8::RegE); 5 },
            Instruction::MOV_H_L => { self.mov(Operand8::RegH, Operand8::RegL); 5 },
            Instruction::MOV_H_M => { self.mov(Operand8::RegH, Operand8::Memory); 7 },
            Instruction::MOV_H_A => { self.mov(Operand8::RegH, Operand8::RegA); 5 },
            Instruction::MOV_L_B => { self.mov(Operand8::RegL, Operand8::RegB); 5 },
            Instruction::MOV_L_C => { self.mov(Operand8::RegL, Operand8::RegC); 5 },
            Instruction::MOV_L_D => { self.mov(Operand8::RegL, Operand8::RegD); 5 },
            Instruction::MOV_L_E => { self.mov(Operand8::RegL, Operand8::RegE); 5 },
            Instruction::MOV_L_H => { self.mov(Operand8::RegL, Operand8::RegH); 5 },
            Instruction::MOV_L_M => { self.mov(Operand8::RegL, Operand8::Memory); 7 },
            Instruction::MOV_L_A => { self.mov(Operand8::RegL, Operand8::RegA); 5 },
            Instruction::MOV_M_B => { self.mov(Operand8::Memory, Operand8::RegB); 7 },
            Instruction::MOV_M_C => { self.mov(Operand8::Memory, Operand8::RegC); 7 },
            Instruction::MOV_M_D => { self.mov(Operand8::Memory, Operand8::RegD); 7 },
            Instruction::MOV_M_E => { self.mov(Operand8::Memory, Operand8::RegE); 7 },
            Instruction::MOV_M_H => { self.mov(Operand8::Memory, Operand8::RegH); 7 },
            Instruction::MOV_M_L => { self.mov(Operand8::Memory, Operand8::RegL); 7 },
            Instruction::MOV_M_A => { self.mov(Operand8::Memory, Operand8::RegH); 7 },
            Instruction::MOV_A_B => { self.mov(Operand8::RegA, Operand8::RegB); 5 },
            Instruction::MOV_A_C => { self.mov(Operand8::RegA, Operand8::RegC); 5 },
            Instruction::MOV_A_D => { self.mov(Operand8::RegA, Operand8::RegD); 5 },
            Instruction::MOV_A_E => { self.mov(Operand8::RegA, Operand8::RegE); 5 },
            Instruction::MOV_A_H => { self.mov(Operand8::RegA, Operand8::RegH); 5 },
            Instruction::MOV_A_L => { self.mov(Operand8::RegA, Operand8::RegL); 5 },
            Instruction::MOV_A_M => { self.mov(Operand8::RegA, Operand8::Memory); 7 },
            Instruction::STAX_B => { self.stax(Operand16::RegPairB); 7 },
            Instruction::STAX_D => { self.stax(Operand16::RegPairD); 7 },
            Instruction::LDAX_B => { self.ldax(Operand16::RegPairB); 7 },
            Instruction::LDAX_D => { self.ldax(Operand16::RegPairD); 7 },
            Instruction::ADD_B => { self.add(Operand8::RegB); 4 },
            Instruction::ADD_C => { self.add(Operand8::RegC); 4 },
            Instruction::ADD_D => { self.add(Operand8::RegD); 4 },
            Instruction::ADD_E => { self.add(Operand8::RegE); 4 },
            Instruction::ADD_H => { self.add(Operand8::RegH); 4 },
            Instruction::ADD_L => { self.add(Operand8::RegL); 4 },
            Instruction::ADD_M => { self.add(Operand8::Memory); 7 },
            Instruction::ADD_A => { self.add(Operand8::RegA); 4 },
            Instruction::ADC_B => { self.adc(Operand8::RegB); 4 },
            Instruction::ADC_C => { self.adc(Operand8::RegC); 4 },
            Instruction::ADC_D => { self.adc(Operand8::RegD); 4 },
            Instruction::ADC_E => { self.adc(Operand8::RegE); 4 },
            Instruction::ADC_H => { self.adc(Operand8::RegH); 4 },
            Instruction::ADC_L => { self.adc(Operand8::RegL); 4 },
            Instruction::ADC_M => { self.adc(Operand8::Memory); 7 },
            Instruction::ADC_A => { self.adc(Operand8::RegA); 4 },
            Instruction::SUB_B => { self.sub(Operand8::RegB); 4 },
            Instruction::SUB_C => { self.sub(Operand8::RegC); 4 },
            Instruction::SUB_D => { self.sub(Operand8::RegD); 4 },
            Instruction::SUB_E => { self.sub(Operand8::RegE); 4 },
            Instruction::SUB_H => { self.sub(Operand8::RegH); 4 },
            Instruction::SUB_L => { self.sub(Operand8::RegL); 4 },
            Instruction::SUB_M => { self.sub(Operand8::Memory); 7 },
            Instruction::SUB_A => { self.sub(Operand8::RegA); 4 },
            Instruction::SBB_B => { self.sbb(Operand8::RegB); 4 },
            Instruction::SBB_C => { self.sbb(Operand8::RegC); 4 },
            Instruction::SBB_D => { self.sbb(Operand8::RegD); 4 },
            Instruction::SBB_E => { self.sbb(Operand8::RegE); 4 },
            Instruction::SBB_H => { self.sbb(Operand8::RegH); 4 },
            Instruction::SBB_L => { self.sbb(Operand8::RegL); 4 },
            Instruction::SBB_M => { self.sbb(Operand8::Memory); 7 },
            Instruction::SBB_A => { self.sbb(Operand8::RegA); 4  },
            Instruction::ANA_B => { self.ana(Operand8::RegB); 4 },
            Instruction::ANA_C => { self.ana(Operand8::RegC); 4 },
            Instruction::ANA_D => { self.ana(Operand8::RegD); 4 },
            Instruction::ANA_E => { self.ana(Operand8::RegE); 4 },
            Instruction::ANA_H => { self.ana(Operand8::RegH); 4 },
            Instruction::ANA_L => { self.ana(Operand8::RegL); 4} ,
            Instruction::ANA_M => { self.ana(Operand8::Memory); 7 },
            Instruction::ANA_A => { self.ana(Operand8::RegA); 4 },
            Instruction::XRA_B => { self.xra(Operand8::RegB); 4 },
            Instruction::XRA_C => { self.xra(Operand8::RegC); 4 },
            Instruction::XRA_D => { self.xra(Operand8::RegD); 4 },
            Instruction::XRA_E => { self.xra(Operand8::RegE); 4 },
            Instruction::XRA_H => { self.xra(Operand8::RegH); 4 },
            Instruction::XRA_L => { self.xra(Operand8::RegL); 4 },
            Instruction::XRA_M => { self.xra(Operand8::Memory); 7 },
            Instruction::XRA_A => { self.xra(Operand8::RegA); 4 },
            Instruction::ORA_B => { self.ora(Operand8::RegB); 4 },
            Instruction::ORA_C => { self.ora(Operand8::RegC); 4 },
            Instruction::ORA_D => { self.ora(Operand8::RegD); 4 },
            Instruction::ORA_E => { self.ora(Operand8::RegE); 4 },
            Instruction::ORA_H => { self.ora(Operand8::RegH); 4 },
            Instruction::ORA_L => { self.ora(Operand8::RegL); 4 },
            Instruction::ORA_M => { self.ora(Operand8::Memory); 7 },
            Instruction::ORA_A => { self.ora(Operand8::RegA); 4 },
            Instruction::CMP_B => { self.cmp(Operand8::RegB); 4 },
            Instruction::CMP_C => { self.cmp(Operand8::RegC); 4 },
            Instruction::CMP_D => { self.cmp(Operand8::RegD); 4 },
            Instruction::CMP_E => { self.cmp(Operand8::RegE); 4 },
            Instruction::CMP_H => { self.cmp(Operand8::RegH); 4 },
            Instruction::CMP_L => { self.cmp(Operand8::RegL); 4 },
            Instruction::CMP_M => { self.cmp(Operand8::Memory); 7 },
            Instruction::CMP_A => { self.cmp(Operand8::RegA); 4 },
            Instruction::RLC => { self.rlc(); 4 },
            Instruction::RRC => { self.rrc(); 4 },
            Instruction::RAL => { self.ral(); 4 },
            Instruction::RAR => { self.rar(); 4 },
            Instruction::PUSH_B => { self.push(Operand16::RegPairB); 11 },
            Instruction::PUSH_D => { self.push(Operand16::RegPairD); 11 },
            Instruction::PUSH_H => { self.push(Operand16::RegPairH); 11 },
            Instruction::PUSH_PSW => { self.push(Operand16::PSW); 11 },
            Instruction::POP_B => { self.pop(Operand16::RegPairB); 10 },
            Instruction::POP_D => { self.pop(Operand16::RegPairD); 10 },
            Instruction::POP_H => { self.pop(Operand16::RegPairH); 10 },
            Instruction::POP_PSW => { self.pop(Operand16::PSW); 10 },
            Instruction::DAD_B => { self.dad(Operand16::RegPairB); 10 },
            Instruction::DAD_D => { self.dad(Operand16::RegPairD); 10 },
            Instruction::DAD_H => { self.dad(Operand16::RegPairH); 10 },
            Instruction::DAD_SP => { self.dad(Operand16::SP); 10 },
            Instruction::INX_B => { self.inx(Operand16::RegPairB); 5 },
            Instruction::INX_D => { self.inx(Operand16::RegPairD); 5 },
            Instruction::INX_H => { self.inx(Operand16::RegPairH); 5 },
            Instruction::INX_SP => { self.inx(Operand16::SP); 5 },
            Instruction::DCX_B => { self.dcx(Operand16::RegPairB); 5 },
            Instruction::DCX_D => { self.dcx(Operand16::RegPairD); 5 },
            Instruction::DCX_H => { self.dcx(Operand16::RegPairH); 5 },
            Instruction::DCX_SP => { self.dcx(Operand16::SP); 5 },
            Instruction::XCHG => { self.xchg(); 4 },
            Instruction::XTHL => { self.xthl(); 18 },
            Instruction::SPHL => { self.sphl(); 5 },
            Instruction::LXI_B => { self.load_imm16(); self.lxi(Operand16::RegPairB); 10 },
            Instruction::LXI_D => { self.load_imm16(); self.lxi(Operand16::RegPairD); 10 },
            Instruction::LXI_H => { self.load_imm16(); self.lxi(Operand16::RegPairH); 10 },
            Instruction::LXI_SP => { self.load_imm16(); self.lxi(Operand16::SP); 10 },
            Instruction::MVI_B => { self.load_imm(); self.mov(Operand8::Immediate, Operand8::RegB); 10 },
            Instruction::MVI_C => { self.load_imm(); self.mov(Operand8::Immediate, Operand8::RegC); 10 },
            Instruction::MVI_D => { self.load_imm(); self.mov(Operand8::Immediate, Operand8::RegD); 10 },
            Instruction::MVI_E => { self.load_imm(); self.mov(Operand8::Immediate, Operand8::RegE); 10 },
            Instruction::MVI_H => { self.load_imm(); self.mov(Operand8::Immediate, Operand8::RegH); 10 },
            Instruction::MVI_L => { self.load_imm(); self.mov(Operand8::Immediate, Operand8::RegL); 10 },
            Instruction::MVI_M => { self.load_imm(); self.mov(Operand8::Immediate, Operand8::Memory); 10 },
            Instruction::MVI_A => { self.load_imm(); self.mov(Operand8::Immediate, Operand8::RegA); 10 },
            Instruction::ADI => { self.load_imm(); self.add(Operand8::Immediate); 7 },
            Instruction::ACI => { self.load_imm(); self.adc(Operand8::Immediate); 7 },
            Instruction::SUI => { self.load_imm(); self.sub(Operand8::Immediate); 7 },
            Instruction::SBI => { self.load_imm(); self.sbb(Operand8::Immediate); 7 },
            Instruction::ANI => { self.load_imm(); self.ana(Operand8::Immediate); 7 },
            Instruction::XRI => { self.load_imm(); self.xra(Operand8::Immediate); 7 },
            Instruction::ORI => { self.load_imm(); self.ora(Operand8::Immediate); 7 },
            Instruction::CPI => { self.load_imm(); self.cmp(Operand8::Immediate); 7 },
            Instruction::STA => { self.load_imm16(); self.sta(); 13 },
            Instruction::LDA => { self.load_imm16(); self.lda(); 13 },
            Instruction::SHLD => { self.load_imm16(); self.shld(); 16 },
            Instruction::LHLD => { self.load_imm16(); self.lhld(); 16 },
            Instruction::PCHL => { self.pchl(); 5 },
            Instruction::JMP => { self.load_imm16(); self.jmp(true); 10 },
            Instruction::JC => { self.load_imm16(); self.jmp(self.registers.status_carry()); 10 },
            Instruction::JNC => { self.load_imm16(); self.jmp(!self.registers.status_carry()); 10 },
            Instruction::JZ => { self.load_imm16(); self.jmp(self.registers.status_zero()); 10 },
            Instruction::JNZ => { self.load_imm16(); self.jmp(!self.registers.status_zero()); 10 },
            Instruction::JM => { self.load_imm16(); self.jmp(self.registers.status_sign()); 10 },
            Instruction::JP => { self.load_imm16(); self.jmp(!self.registers.status_sign()); 10 },
            Instruction::JPE => { self.load_imm16(); self.jmp(self.registers.status_parity()); 10 },
            Instruction::JPO => { self.load_imm16(); self.jmp(!self.registers.status_parity()); 10 },
            Instruction::CALL => { self.load_imm16(); self.call(true); 17 },
            Instruction::CC => { self.load_imm16(); if self.call(self.registers.status_carry()) {17} else {11} },
            Instruction::CNC => { self.load_imm16(); if self.call(!self.registers.status_carry()) {17} else {11} },
            Instruction::CZ => { self.load_imm16(); if self.call(self.registers.status_zero()) {17} else {11} },
            Instruction::CNZ => { self.load_imm16(); if self.call(!self.registers.status_zero()) {17} else {11} },
            Instruction::CM => { self.load_imm16(); if self.call(self.registers.status_sign()) {17} else {11} },
            Instruction::CP => { self.load_imm16(); if self.call(!self.registers.status_sign()) {17} else {11} },
            Instruction::CPE => { self.load_imm16(); if self.call(self.registers.status_parity()) {17} else {11} },
            Instruction::CPO => { self.load_imm16(); if self.call(!self.registers.status_parity()) {17} else {11} },
            Instruction::RET => { self.ret(true); 10 },
            Instruction::RC => { self.ret(self.registers.status_carry()); 10 },
            Instruction::RNC => { self.ret(!self.registers.status_carry()); 10 },
            Instruction::RZ => { self.ret(self.registers.status_zero()); 10 },
            Instruction::RNZ => { self.ret(!self.registers.status_zero()); 10 },
            Instruction::RM => { self.ret(self.registers.status_sign()); 10 },
            Instruction::RP => { self.ret(!self.registers.status_sign()); 10 },
            Instruction::RPE => { self.ret(self.registers.status_parity()); 10 },
            Instruction::RPO => { self.ret(!self.registers.status_parity()); 10 },
            Instruction::RST_1 => { self.rst(0); 11 },
            Instruction::RST_2 => { self.rst(1); 11 },
            Instruction::RST_3 => { self.rst(2); 11 },
            Instruction::RST_4 => { self.rst(3); 11 },
            Instruction::RST_5 => { self.rst(4); 11 },
            Instruction::RST_6 => { self.rst(5); 11 },
            Instruction::RST_7 => { self.rst(6); 11 },
            Instruction::RST_8 => { self.rst(7); 11 },
            Instruction::EI => { self.ei(); 4 },
            Instruction::DI => { self.di(); 4 },
            Instruction::IN => { self.input(); 10 },
            Instruction::OUT => { self.out(); 10 },
            Instruction::HLT => { self.hlt(); 7 },
            _ => {4}
        }
    }

    fn load_imm(&mut self) {
        let val = self.fetch_instruction() as u8;
        self.registers.set_z(val);
    }

    fn load_imm16(&mut self) {
        let val = self.fetch_instruction() as u8;
        self.registers.set_z(val);
        
        let val = self.fetch_instruction() as u8;
        self.registers.set_w(val);
    }

    fn get_src(&mut self, src: Operand8) -> u8 {
        match src {
            Operand8::RegB => { self.registers.b() },
            Operand8::RegC => { self.registers.c() },
            Operand8::RegD => { self.registers.d() },
            Operand8::RegE => { self.registers.e() },
            Operand8::RegH => { self.registers.h() },
            Operand8::RegL => { self.registers.l() },
            Operand8::Memory => { self.memory[self.registers.pair_h() as usize] }
            Operand8::RegA => { self.registers.accumulator() },
            Operand8::Immediate => { self.registers.z() }
        }
    }

    fn write_dst(&mut self, dst: Operand8, val: u8) {
        match dst {
            Operand8::RegB => { self.registers.set_b(val) },
            Operand8::RegC => { self.registers.set_c(val) },
            Operand8::RegD => { self.registers.set_d(val) },
            Operand8::RegE => { self.registers.set_e(val) },
            Operand8::RegH => { self.registers.set_h(val) },
            Operand8::RegL => { self.registers.set_l(val) },
            Operand8::Memory => { self.memory[self.registers.pair_h() as usize] = val; },
            Operand8::RegA => { self.registers.set_accumulator(val) },
            Operand8::Immediate => {/* INVALID */} 
        }
    }

    fn get_src_16(&mut self, src: Operand16) -> u16 {
        match src {
            Operand16::PSW => { self.registers.psw() },
            Operand16::RegPairB => { self.registers.pair_b() },
            Operand16::RegPairD => { self.registers.pair_d() },
            Operand16::RegPairH => { self.registers.pair_h() },
            Operand16::Immediate => { self.registers.pair_w() },
            Operand16::SP => { self.registers.sp() }
        }
    }

    fn write_dst_16(&mut self, src: Operand16, val: u16) {
        match src {
            Operand16::PSW => { self.registers.set_psw(val) },
            Operand16::RegPairB => { self.registers.set_pair_b(val) },
            Operand16::RegPairD => { self.registers.set_pair_d(val) },
            Operand16::RegPairH => { self.registers.set_pair_h(val) },
            Operand16::Immediate => {/* INVALID */},
            Operand16::SP => { self.registers.set_sp(val) }
        }
    }

    fn set_condition(&mut self, val: u8, carry: bool, aux_carry: bool) {
        self.registers.set_status_zero(val == 0);
        self.registers.set_status_sign(val & 0x80 != 0);
        self.registers.set_status_aux_carry(parity_even(val));
        self.registers.set_status_carry(carry);
        self.registers.set_status_aux_carry(aux_carry);
    }

    // Set Carry
    fn stc(&mut self) {
        self.registers.set_status_carry(true);
    }

    // Complement Carry
    fn cmc(&mut self) {
        self.registers.set_status_carry(!self.registers.status_carry());
    }

    // Increment Register or Memory
    fn inr(&mut self, reg: Operand8) {
        let val = self.get_src(reg);
        let val = val.wrapping_add(1);
        self.write_dst(reg, val);
        self.set_condition(val, val == 0, val % 0x10 == 0);
    }

    // Decrement Register or Memory
    fn dcr(&mut self, reg: Operand8) {
        // Note: This is equivalent in Intel8080 to adding 0xFF to the value
        // due to the usage of 2s complement representation. Carry bits will be 
        // set accordingly.
        let orig_val = self.get_src(reg);
        let new_val = orig_val.wrapping_sub(1);
        self.write_dst(reg, new_val);
        self.set_condition(new_val, new_val != 255, orig_val & 0xF != 0);
    }

    // Complement Accumulator
    fn cma(&mut self) {
        self.registers.set_accumulator(!self.registers.accumulator());
    }

    // Decimal Adjust Accumulator
    fn daa(&mut self) {
        let mut val: u8 = self.registers.accumulator();
        
        let aux_carry = val & 0xF > 0x9;
        if aux_carry || self.registers.status_aux_carry() {
            val += 0x6;
        }

        let carry = val & 0xF0 > 0x90;
        if carry || self.registers.status_carry() {
            val += 0x60
        }

        self.registers.set_accumulator(val);
        self.set_condition(val, carry, aux_carry);
    }

    // Move
    fn mov(&mut self, src: Operand8, dst: Operand8) {
        let src = self.get_src(src);
        self.write_dst(dst, src);
    }

    // Store Accumulator
    fn stax(&mut self, dst: Operand16) {
        match dst {
            Operand16::RegPairB => {
                self.write_memory(self.registers.pair_b(), self.registers.accumulator());
            },
            Operand16::RegPairD => {
                self.write_memory(self.registers.pair_d(), self.registers.accumulator());
            },
            _ => {/* INVALID */}
        }
    }

    // Load Accumulator
    fn ldax(&mut self, dst: Operand16) {
        match dst {
            Operand16::RegPairB => {
                self.registers.set_accumulator(self.read_memory(self.registers.pair_b()));
            },
            Operand16::RegPairD => {
                self.registers.set_accumulator(self.read_memory(self.registers.pair_d()));
            },
            _ => {/* INVALID */}
        }
    }

    // ADD Register or Memory to Accumulator
    fn add(&mut self, src: Operand8) {
        let old_val: u8 = self.registers.accumulator();
        let new_val: u8 = old_val.wrapping_add(self.get_src(src)); 
        
        let carry: bool = check_carry(old_val, new_val);
        let aux_carry: bool = check_aux_carry(old_val, new_val);

        self.registers.set_accumulator(new_val as u8);
        self.set_condition(new_val as u8, carry, aux_carry);
    }

    // ADD Register or Memory to Accumulator With Carry
    fn adc(&mut self, src: Operand8) {
        let old_val: u8 = self.registers.accumulator();
        let new_val: u8 = old_val.wrapping_add(self.get_src(src))
            .wrapping_add(self.registers.status_carry() as u8); 
        
        let carry: bool = check_carry(old_val, new_val);
        let aux_carry: bool = check_aux_carry(old_val, new_val);

        self.registers.set_accumulator(new_val);
        self.set_condition(new_val, carry, aux_carry);
    }

    // Subtract Register or Memory From Accumulator
    fn sub(&mut self, src: Operand8) {
        let old_val = self.registers.accumulator();
        let new_val = old_val.wrapping_add(twos_complement(self.get_src(src)));

        let carry: bool = check_carry(old_val, new_val);
        let aux_carry: bool = check_aux_carry(old_val, new_val);

        self.registers.set_accumulator(new_val);
        self.set_condition(new_val, carry, aux_carry);
    } 

    // Subtract Register or Memory From Accumulator With Borrow
    fn sbb(&mut self, src: Operand8) {
        let old_val = self.registers.accumulator();
        let new_val = twos_complement(self.get_src(src)
            .wrapping_add(self.registers.status_carry() as u8));
        let new_val = old_val.wrapping_add(new_val);

        let carry: bool = check_carry(old_val, new_val);
        let aux_carry: bool = check_aux_carry(old_val, new_val);

        self.registers.set_accumulator(new_val);
        self.set_condition(new_val, carry, aux_carry);
    }

    // Logical and Register or Memory With Accumulator
    fn ana(&mut self, src: Operand8) {
        let val: u8 = self.registers.accumulator() & self.get_src(src);
        self.set_condition(val, false, false);
        self.registers.set_accumulator(val);
    }

    // Logical Exclusive-Or Register or Memory with Accumulator
    fn xra(&mut self, src: Operand8) {
        let val: u8 = self.registers.accumulator() ^ self.get_src(src);
        self.set_condition(val, false, false);
        self.registers.set_accumulator(val);
    }

    // Logical or Register or Memory with Accumulator
    fn ora(&mut self, src: Operand8) {
        let val: u8 = self.registers.accumulator() | self.get_src(src);
        self.set_condition(val, false, false);
        self.registers.set_accumulator(val);
    }

    // Compare Register or Memory with Accumulator
    fn cmp(&mut self, src:Operand8) {
        let old_val = self.registers.accumulator();
        let new_val = old_val.wrapping_add(twos_complement(self.get_src(src)));

        let carry: bool = check_carry(old_val, new_val);
        let aux_carry: bool = check_aux_carry(old_val, new_val);

        self.set_condition(new_val, carry, aux_carry)
    }

    // Rotate Accumulator Left
    fn rlc(&mut self) {
        let val: u8 = self.registers.accumulator();
        
        // check left-most bit
        if val & 0x80 != 0 {
            self.registers.set_status_carry(true);        
        }
        else {
            self.registers.set_status_carry(false);
        }
       
        // shift left and set right-most bit to the previous left-most bit
        let val: u8 = (val << 1) | if self.registers.status_carry() {1} else {0};
        
        self.registers.set_accumulator(val);
    }

    // Rotate Accumulator Right
    fn rrc(&mut self) {
        let val: u8 = self.registers.accumulator();
        
        // check right-most bit
        if val & 0x01 != 0 {
            self.registers.set_status_carry(true);        
        }
        else {
            self.registers.set_status_carry(false);
        }
       
        // shift right and set left-most bit to the previous right-most bit
        let val: u8 = (val >> 1) | if self.registers.status_carry() {0x80} else {0};
        
        self.registers.set_accumulator(val);
    }

    // Rotate Accumulator Left Through Carry
    fn ral(&mut self) {
        let val: u8 = self.registers.accumulator();
        let old_carry: bool = self.registers.status_carry();

        // set carry to left-most bit
        self.registers.set_status_carry(val & 0x80 != 0);        
       
        // shift left and set right-most bit to the previous left-most bit
        let val: u8 = (val << 1) | (if old_carry {1} else {0});
        
        self.registers.set_accumulator(val);
    }

    // Rotate Accumulator Right Through Carry
    fn rar(&mut self) {
        let val: u8 = self.registers.accumulator();
        let old_carry: bool = self.registers.status_carry();

        // set carry to right most bit
        self.registers.set_status_carry(val & 0x01 != 0);        
       
        // shift right and set left-most bit to the old carry bit value
        let val: u8 = (val >> 1) | (if old_carry {0x80} else {0});
        
        self.registers.set_accumulator(val);
    }

    // Push Data Onto Stack
    fn push(&mut self, src: Operand16) {
        let first_register = match src {
            Operand16::RegPairB => { self.registers.b() },
            Operand16::RegPairD => { self.registers.d() },
            Operand16::RegPairH => { self.registers.h() },
            Operand16::PSW => { self.registers.status() },
            _ => { 0 }
        };
        
        let second_register = match src {
            Operand16::RegPairB => { self.registers.c() },
            Operand16::RegPairD => { self.registers.e() },
            Operand16::RegPairH => { self.registers.l() },
            Operand16::PSW => { self.registers.accumulator() },
            _ => { 0 }
        };
        
        self.push_stack(first_register);
        self.push_stack(second_register);
    }

    // Pop Data Off Stack
    fn pop(&mut self, dst: Operand16) {
        let second_register = self.pop_stack();
        let first_register = self.pop_stack();

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
            _ => { /* INVALID */ }
        };
    }

    // Double Add
    fn dad(&mut self, src: Operand16) {
        let val = self.get_src_16(src);
        
        let carry = (val & 0x80) | (self.registers.pair_h() & 0x80) != 0;
        self.registers.set_status_carry(carry);
        
        self.registers.set_pair_h(val + self.registers.pair_h());
    }

    // Increment Register Pair
    fn inx(&mut self, src: Operand16) {
        let val = self.get_src_16(src);
        self.write_dst_16(src, val + 1);
    }

    // Decrement Register Pair
    fn dcx(&mut self, src: Operand16) {
        let val = self.get_src_16(src);
        self.write_dst_16(src, val - 1);
    }

    // Exchange Registers
    fn xchg(&mut self) {
        let temp = self.registers.pair_h();
        self.registers.set_pair_h(self.registers.pair_d());
        self.registers.set_pair_d(temp);
    }

    // Exchange Stack
    fn xthl(&mut self) {
        let temp = self.registers.l();
        self.registers.set_l(self.memory[self.registers.sp() as usize + 1]);
        self.memory[self.registers.sp() as usize + 1] = temp;
    }

    // Load SP From H and L
    fn sphl(&mut self) {
        self.registers.set_sp(self.registers.pair_h());
    }

    // Load Register Pair Immediate
    fn lxi(&mut self, dst: Operand16) {
        let val = self.registers.pair_w();
        self.write_dst_16(dst, val);
    }

    // Store Accumulator Direct
    fn sta(&mut self) {
        self.write_memory(self.registers.pair_w(), self.registers.accumulator());
    }

    // Load Accumulator Direct
    fn lda(&mut self) {
        self.registers.set_accumulator(self.read_memory(self.registers.pair_w()));
    }

    // Store H and L Direct
    fn shld(&mut self) {
        self.write_memory(self.registers.pair_w(), self.registers.l());
        self.write_memory(self.registers.pair_w() + 1, self.registers.h());
    }

    // Load H and L Direct
    fn lhld(&mut self) {
        self.registers.set_l(self.read_memory(self.registers.pair_w()));
        self.registers.set_h(self.read_memory(self.registers.pair_w()));
    }

    // Load Program Counter
    fn pchl(&mut self) {
        self.registers.set_pc(self.registers.pair_h());
    }

    // Jump
    fn jmp(&mut self, condition:  bool) {
        if condition {
            self.registers.set_pc(self.registers.pair_w());
        }
    }

    // Call
    fn call(&mut self, condition: bool) -> bool {
        if condition {
            self.push_stack((self.registers.pc() >> 8) as u8);
            self.push_stack((self.registers.pc() & 0xFF) as u8);
            self.jmp(true);
            return true;
        }
        return false
    }
    // Return
    fn ret(&mut self, condition: bool) {
        if condition {
            let new_pc:u16 = make_u16(self.pop_stack(),
                                      self.pop_stack());
            self.registers.set_pc(new_pc);
        }
    }
    fn rst(&mut self, exp: u8) {
        self.registers.set_w(0);
        self.registers.set_z(exp << 3);
        self.call(true);
    }
    // Enable Interrupts
    fn ei(&mut self) {
        self.inte = true;
    }

    // Disable Interrupts
    fn di(&mut self) {
        self.inte = false;
    }

    // Input
    fn input(&self) {
        self.start_machine_cycle(MachineCycle::InputRead);
    }

    // Output
    fn out(&self) {
        self.start_machine_cycle(MachineCycle::OutputWrite);
    }

    // Halt
    fn hlt(&mut self) {
        self.stopped = true;
    }
}
