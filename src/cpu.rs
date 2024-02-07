use std::usize;

use crate::registers::Registers as Registers;
use crate::memory::MemoryAccess as MemoryAccess;
use crate::utils::*;

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

#[derive(Copy, Clone)]
pub struct Port {
    input_callback: fn()->u8,
    output_callback: fn(u8)
}

impl Port {
    pub fn new(input_callback: fn()->u8, output_callback: fn(u8)) -> Self {
        Self {
            input_callback,
            output_callback
        }
    }
}

pub struct Intel8080 {
    registers: Registers,
    interrupt_instruction: Option<Instruction>,
    port_callbacks: [Option<Port>; 256],
    stopped: bool,
    inte: bool
}

impl Intel8080 {
    pub fn new() -> Self {
        Self {
            registers: Registers::new(),
            port_callbacks: [None; 256],
            interrupt_instruction: None,
            stopped: false,
            inte: false,
        }
    }

    pub fn set_port(&mut self, port_index: u8, port: Port) {
        self.port_callbacks[port_index as usize] = Some(port);
    }

    pub fn unset_port(&mut self, port_index: u8) {
        self.port_callbacks[port_index as usize] = None;
    }

    pub fn step(&mut self, memory: &mut impl MemoryAccess) -> usize {
        let instruction = self.fetch_instruction(memory);
        self.do_instruction(instruction, memory)
    }

    pub fn interrupt(&mut self, instruction: Instruction) {
        if self.inte {
            self.interrupt_instruction = Some(instruction)
        }
    }

    fn fetch_instruction(&mut self, memory: &impl MemoryAccess) -> Instruction {
        let instruction = match self.interrupt_instruction.take() {
            Some(x) => x,
            None => { self.registers.set_pc(self.registers.pc() + 1);
                        Instruction::from(memory.read(self.registers.pc() - 1))
            }
        };

        return instruction
    }


    fn pop_stack(&mut self, memory: &impl MemoryAccess) -> u8 {
        self.registers.set_sp(self.registers.sp() + 1);
        memory.read(self.registers.sp() - 1)
    }

    fn push_stack(&mut self, val: u8, memory: &mut impl MemoryAccess) {
        self.registers.set_sp(self.registers.sp() - 1);
        memory.write(self.registers.sp() - 1, val);
    }

    fn do_instruction(&mut self, instruction: Instruction, memory: &mut impl MemoryAccess) -> usize {
        let mut cycles = 4;

        if instruction == Instruction::CMC {
            cycles = self.cmc();
        }
        else if instruction == Instruction::STC {
            cycles = self.stc();
        }
        else if instruction as u8 & 0xC7 == 0x04 {
            let operand = (instruction as u8 & 0x38) >> 3;
            cycles = self.inr(operand, memory);
        }
        else if instruction as u8 & 0xC7 == 0x05 {
            let operand = (instruction as u8 & 0x38) >> 3;
            cycles = self.dcr(operand, memory);
        }
        else if instruction == Instruction::CMA {
            cycles = self.cma();
        }
        else if instruction == Instruction::DAA {
            cycles = self.daa();
        }
        else if instruction as u8 & 0xC0 == 0x40 {
            let dst = (instruction as u8 & 0x38) >> 3;
            let src = instruction as u8 & 0x07;
            cycles = self.mov(dst, src, memory);
        }
        else if instruction as u8 & 0xEF == 0x02 {
            let dst = (instruction as u8 & 0x10) >> 4;
            cycles = self.stax(dst, memory);
        }
        else if instruction as u8 & 0xEF == 0x0A {
            let dst = (instruction as u8 & 0x10) >> 4;
            cycles = self.ldax(dst, memory);
        }
        else if instruction as u8 & 0xF8 == 0x80 {
            let reg = instruction as u8 & 0x7;
            cycles = self.add(reg, memory);
        }
        else if instruction as u8 & 0xF8 == 0x88 {
            let reg = instruction as u8 & 0x7;
            cycles = self.adc(reg, memory);
        }
        else if instruction as u8 & 0xF8 == 0x90 {
            let reg = instruction as u8 & 0x7;
            cycles = self.sub(reg, memory);
        }
        else if instruction as u8 & 0xF8 == 0x98 {
            let reg = instruction as u8 & 0x7;
            cycles = self.sbb(reg, memory);
        }
        else if instruction as u8 & 0xF8 == 0xA0 {
            let reg = instruction as u8 & 0x7;
            cycles = self.ana(reg, memory);
        }
        else if instruction as u8 & 0xF8 == 0xA8 {
            let reg = instruction as u8 & 0x7;
            cycles = self.xra(reg, memory);
        }
        else if instruction as u8 & 0xF8 == 0xB0 {
            let reg = instruction as u8 & 0x7;
            cycles = self.ora(reg, memory);
        }
        else if instruction as u8 & 0xF8 == 0xB8 {
            let reg = instruction as u8 & 0x7;
            cycles = self.cmp(reg, memory);
        }
        else if instruction == Instruction::RLC {
            cycles = self.rlc();
        }
        else if instruction == Instruction::RRC {
            cycles = self.rrc();
        }
        else if instruction == Instruction::RAL {
            cycles = self.ral();
        }
        else if instruction == Instruction::RAR {
            cycles = self.rar();
        }
        else if instruction as u8 & 0xCF == 0xC5 {
            let dst = (instruction as u8 & 0x30) >> 4;
            cycles = self.push(dst, memory);
        }
        else if instruction as u8 & 0xCF == 0xC1 {
            let dst = (instruction as u8 & 0x30) >> 4;
            cycles = self.pop(dst, memory);
        }
        else if instruction as u8 & 0xCF == 0x09 {
            let src = (instruction as u8 & 0x30) >> 4;
            cycles = self.dad(src);
        }
        else if instruction as u8 & 0xCF == 0x03 {
            let src = (instruction as u8 & 0x30) >> 4;
            cycles = self.inx(src);
        }
        else if instruction as u8 & 0xCF == 0x0B {
            let src = (instruction as u8 & 0x30) >> 4;
            cycles = self.dcx(src);
        }
        else if instruction == Instruction::XCHG {
            cycles = self.xchg();
        }
        else if instruction == Instruction::XTHL {
            cycles = self.xthl(memory);
        }
        else if instruction == Instruction::SPHL {
            cycles = self.sphl();
        }
        else if instruction as u8 & 0xCF == 0x1 {
            let reg = (instruction as u8 & 0x30) >> 4;
            cycles = self.lxi(reg, memory);
        }
        else if instruction as u8 & 0xCF == 0x06 {
            let reg = (instruction as u8 & 0x38) >> 3;
            cycles = self.mov(reg, Operand8::Immediate as u8, memory);
        }
        else if instruction == Instruction::ADI {
            cycles = self.add(Operand8::Immediate as u8, memory);
        }
        else if instruction == Instruction::ACI {
            cycles = self.adc(Operand8::Immediate as u8, memory);
        }
        else if instruction == Instruction::SUI {
            cycles = self.sub(Operand8::Immediate as u8, memory);
        }
        else if instruction == Instruction::SBI {
            cycles = self.sbb(Operand8::Immediate as u8, memory);
        }
        else if instruction == Instruction::ANI {
            cycles = self.ana(Operand8::Immediate as u8, memory);
        }
        else if instruction == Instruction::XRI {
            cycles = self.xra(Operand8::Immediate as u8, memory);
        }
        else if instruction == Instruction::ORI {
            cycles = self.ora(Operand8::Immediate as u8, memory);
        }
        else if instruction == Instruction::CPI {
            cycles = self.cmp(Operand8::Immediate as u8, memory);
        }
        else if instruction == Instruction::STA {
            cycles = self.sta(memory);
        }
        else if instruction == Instruction::LDA {
            cycles = self.lda(memory);
        }
        else if instruction == Instruction::SHLD {
            cycles = self.shld(memory);
        }
        else if instruction == Instruction::LHLD {
            cycles = self.lhld(memory);
        }
        else if instruction == Instruction::JMP {
            cycles = self.jmp(8, memory);
        }
        else if instruction as u8 & 0xC7 == 0xC2 {
            cycles = self.jmp((instruction as u8 & 0x38) >> 3, memory);
        }
        else if instruction == Instruction::PCHL {
            cycles = self.pchl();
        }
        else if instruction == Instruction::CALL {
            cycles = self.call(8, memory);
        }
        else if instruction as u8 & 0xC7 == 0xC4 {
            cycles = self.call((instruction as u8 & 0x38) >> 3, memory);
        }
        else if instruction == Instruction::RET {
            cycles = self.ret(8, memory);
        }
        else if instruction as u8 & 0xC7 == 0xC0 {
            cycles = self.ret((instruction as u8 & 0x38) >> 3, memory);
        } 
        
        cycles
    }

    fn load_imm(&mut self, memory: &impl MemoryAccess) {
        let val = self.fetch_instruction(memory) as u8;
        self.registers.set_z(val);
    }

    fn load_imm16(&mut self, memory: &impl MemoryAccess) {
        let val = self.fetch_instruction(memory) as u8;
        self.registers.set_z(val);
        
        let val = self.fetch_instruction(memory) as u8;
        self.registers.set_w(val);
    }

    fn get_src(&mut self, src: Operand8, memory: &impl MemoryAccess) -> u8 {
        match src {
            Operand8::RegB => { self.registers.b() },
            Operand8::RegC => { self.registers.c() },
            Operand8::RegD => { self.registers.d() },
            Operand8::RegE => { self.registers.e() },
            Operand8::RegH => { self.registers.h() },
            Operand8::RegL => { self.registers.l() },
            Operand8::Memory => { memory.read(self.registers.pair_h()) }
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
            Operand8::Memory => { memory.write(self.registers.pair_h(), val); },
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

    fn set_condition(&mut self, val: u8, carry: bool, aux_carry: bool) {
        self.registers.set_status_zero(val == 0);
        self.registers.set_status_sign(val & 0x80 != 0);
        self.registers.set_status_parity(parity_even(val));
        self.registers.set_status_carry(carry);
        self.registers.set_status_aux_carry(aux_carry);
    }

    // Set Carry
    fn stc(&mut self) -> usize {
        self.registers.set_status_carry(true);
        4
    }

    // Complement Carry
    fn cmc(&mut self) -> usize {
        self.registers.set_status_carry(!self.registers.status_carry());
        4
    }

    // Increment Register or Memory
    fn inr(&mut self, reg: u8, memory: &mut impl MemoryAccess) -> usize {
        let reg = Operand8::from(reg);
        let val = self.get_src(reg, memory);
        let val = val.wrapping_add(1);
        self.write_dst(reg, val, memory);
        self.set_condition(val, val == 0, val % 0x10 == 0);

        if reg == Operand8::Memory {
            return 10
        }
        return 5
    }

    // Decrement Register or Memory
    fn dcr(&mut self, reg: u8, memory: &mut impl MemoryAccess) -> usize {
        // Note: This is equivalent in Intel8080 to adding 0xFF to the value
        // due to the usage of 2s complement representation. Carry bits will be 
        // set accordingly.

        let reg = Operand8::from(reg);
        let orig_val = self.get_src(reg, memory);
        let new_val = orig_val.wrapping_sub(1);
        self.write_dst(reg, new_val, memory);
        self.set_condition(new_val, new_val != 255, orig_val & 0xF != 0);

        if reg == Operand8::Memory {
            return 10
        }
        return 5
    }

    // Complement Accumulator
    fn cma(&mut self) -> usize {
        self.registers.set_accumulator(!self.registers.accumulator());
        4
    }

    // Decimal Adjust Accumulator
    fn daa(&mut self) -> usize {
        let mut val: u8 = self.registers.accumulator();
        
        let aux_carry = val & 0xF > 0x9;
        if aux_carry || self.registers.status_aux_carry() {
            val = val.wrapping_add(0x6);
        }

        let carry = val & 0xF0 > 0x90;
        if carry || self.registers.status_carry() {
            val = val.wrapping_add(0x60);
        }

        self.registers.set_accumulator(val);
        self.set_condition(val, carry, aux_carry);
        4
    }

    // Move
    fn mov(&mut self, dst: u8, src: u8, memory: &mut impl MemoryAccess) -> usize {
        let dst = Operand8::from(dst);
        let src = Operand8::from(src);
        let val = self.get_src(src, memory);
        self.write_dst(dst, val, memory);

        if (dst == Operand8::Memory) || (src == Operand8::Memory) || (src == Operand8::Immediate) {
            return 7
        }
        return 5
    }

    // Store Accumulator
    fn stax(&mut self, dst: u8, memory: &mut impl MemoryAccess) -> usize {
        match dst {
            0 => {
                memory.write(self.registers.pair_b(), self.registers.accumulator());
            },
            1 => {
                memory.write(self.registers.pair_d(), self.registers.accumulator());
            },
            _ => { panic!("Invalid dst passed to STAX: {}!", dst) }
        };

        7
    }

    // Load Accumulator
    fn ldax(&mut self, dst: u8, memory: &mut impl MemoryAccess) -> usize {
        match dst {
            0 => {
                self.registers.set_accumulator(memory.read(self.registers.pair_b()));
            },
            1 => {
                self.registers.set_accumulator(memory.read(self.registers.pair_d()));
            },
            _ => { panic!("Invalid dst passed to LDAX: {}!", dst) }
        };

        7
    }

    // ADD Register or Memory to Accumulator
    fn add(&mut self, src: u8, memory: &impl MemoryAccess) -> usize {
        let src = Operand8::from(src);

        let old_val: u8 = self.registers.accumulator();
        let new_val: u8 = old_val.wrapping_add(self.get_src(src, memory)); 
        
        let carry: bool = check_carry(old_val, new_val);
        let aux_carry: bool = check_aux_carry(old_val, new_val);

        self.registers.set_accumulator(new_val as u8);
        self.set_condition(new_val as u8, carry, aux_carry);
        
        if (src == Operand8::Memory) || (src == Operand8::Immediate) {
            return 7
        }
        return 4
    }

    // ADD Register or Memory to Accumulator With Carry
    fn adc(&mut self, src: u8, memory: &impl MemoryAccess) -> usize {
        let src = Operand8::from(src);
        
        let old_val: u8 = self.registers.accumulator();
        let new_val: u8 = old_val.wrapping_add(self.get_src(src, memory))
            .wrapping_add(self.registers.status_carry() as u8); 
        
        let carry: bool = check_carry(old_val, new_val);
        let aux_carry: bool = check_aux_carry(old_val, new_val);

        self.registers.set_accumulator(new_val);
        self.set_condition(new_val, carry, aux_carry);
        
        if (src == Operand8::Memory) || (src == Operand8::Immediate) {
            return 7
        }
        return 4
    }

    // Subtract Register or Memory From Accumulator
    fn sub(&mut self, src: u8, memory: &impl MemoryAccess) -> usize {
        let src = Operand8::from(src);

        let old_val = self.registers.accumulator();
        let new_val = old_val.wrapping_add(twos_complement(self.get_src(src, memory)));

        let carry: bool = check_carry(old_val, new_val);
        let aux_carry: bool = check_aux_carry(old_val, new_val);

        self.registers.set_accumulator(new_val);
        self.set_condition(new_val, carry, aux_carry);
        
        if (src == Operand8::Memory) || (src == Operand8::Immediate) {
            return 7
        }
        return 4
    } 

    // Subtract Register or Memory From Accumulator With Borrow
    fn sbb(&mut self, src: u8, memory: &impl MemoryAccess) -> usize {
        let src = Operand8::from(src); 

        let old_val = self.registers.accumulator();
        let new_val = twos_complement(self.get_src(src, memory)
            .wrapping_add(self.registers.status_carry() as u8));
        let new_val = old_val.wrapping_add(new_val);

        let carry: bool = check_carry(old_val, new_val);
        let aux_carry: bool = check_aux_carry(old_val, new_val);

        self.registers.set_accumulator(new_val);
        self.set_condition(new_val, carry, aux_carry);
        
        if (src == Operand8::Memory) || (src == Operand8::Immediate) {
            return 7
        }
        return 4
    }

    // Logical and Register or Memory With Accumulator
    fn ana(&mut self, src: u8, memory: &impl MemoryAccess) -> usize {
        let src = Operand8::from(src);

        let val: u8 = self.registers.accumulator() & self.get_src(src, memory);
        self.set_condition(val, false, false);
        self.registers.set_accumulator(val);
        
        if (src == Operand8::Memory) || (src == Operand8::Immediate) {
            return 7
        }
        return 4
    }

    // Logical Exclusive-Or Register or Memory with Accumulator
    fn xra(&mut self, src: u8, memory: &impl MemoryAccess) -> usize {
        let src = Operand8::from(src); 

        let val: u8 = self.registers.accumulator() ^ self.get_src(src, memory);
        self.set_condition(val, false, false);
        self.registers.set_accumulator(val);
        
        if (src == Operand8::Memory) || (src == Operand8::Immediate) {
            return 7
        }
        return 4
    }

    // Logical or Register or Memory with Accumulator
    fn ora(&mut self, src: u8, memory: &impl MemoryAccess) -> usize {
        let src = Operand8::from(src);

        let val: u8 = self.registers.accumulator() | self.get_src(src, memory);
        self.set_condition(val, false, false);
        self.registers.set_accumulator(val);
        
        if (src == Operand8::Memory) || (src == Operand8::Immediate) {
            return 7
        }
        return 4
    }

    // Compare Register or Memory with Accumulator
    fn cmp(&mut self, src: u8, memory: &impl MemoryAccess) -> usize {
        let src = Operand8::from(src);

        let old_val = self.registers.accumulator();
        let new_val = old_val.wrapping_add(twos_complement(self.get_src(src, memory)));

        let carry: bool = check_carry(old_val, new_val);
        let aux_carry: bool = check_aux_carry(old_val, new_val);

        self.set_condition(new_val, carry, aux_carry);
        
        if (src == Operand8::Memory) || (src == Operand8::Immediate) {
            return 7
        }
        return 4
    }

    // Rotate Accumulator Left
    fn rlc(&mut self) -> usize {
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
        4
    }

    // Rotate Accumulator Right
    fn rrc(&mut self) -> usize {
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
        4
    }

    // Rotate Accumulator Left Through Carry
    fn ral(&mut self) -> usize {
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
    fn rar(&mut self) -> usize {
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
    fn push(&mut self, src: u8, memory: &mut impl MemoryAccess) -> usize {
        let first_register = match src {
            0 => { self.registers.b() },
            1 => { self.registers.d() },
            2 => { self.registers.h() },
            3 => { self.registers.status() },
            _ => { panic!("Invalid src passed to PUSH: {}!", src) }
        };
        
        let second_register = match src {
            0 => { self.registers.c() },
            1 => { self.registers.e() },
            2 => { self.registers.l() },
            3 => { self.registers.accumulator() },
            _ => { unreachable!() }
        };
        
        self.push_stack(first_register, memory);
        self.push_stack(second_register, memory);
        11
    }

    // Pop Data Off Stack
    fn pop(&mut self, dst: u8, memory: &impl MemoryAccess) -> usize {
        let second_register = self.pop_stack(memory);
        let first_register = self.pop_stack(memory);

        match dst {
            0 => {
                self.registers.set_b(first_register);
                self.registers.set_c(second_register);
            },
            1 => {
                self.registers.set_d(first_register);
                self.registers.set_e(second_register);
            },
            2 => {
                self.registers.set_h(first_register);
                self.registers.set_l(second_register);
            },
            3 => {
                self.registers.set_status(first_register);
                self.registers.set_accumulator(second_register);
            },
            _ => { panic!("Invalid dst passed to POP: {}!", dst) }
        };
        10
    }

    // Double Add
    fn dad(&mut self, src: u8) -> usize {
        let src = match src {
            0 => Operand16::RegPairB,
            1 => Operand16::RegPairD,
            2 => Operand16::RegPairH,
            3 => Operand16::SP,
            _ => { panic!("Invalid src passed to DAD: {}!", src); }
        };

        let val = self.get_src_16(src);
        
        let carry = (val & 0x80) | (self.registers.pair_h() & 0x80) != 0;
        self.registers.set_status_carry(carry);
        
        self.registers.set_pair_h(val + self.registers.pair_h());
        10
    }

    // Increment Register Pair
    fn inx(&mut self, src: u8) -> usize {
        let src = match src {
            0 => Operand16::RegPairB,
            1 => Operand16::RegPairD,
            2 => Operand16::RegPairH,
            3 => Operand16::SP,
            _ => { panic!("Invalid src passed to INX: {}!", src); }
        };
        
        let val = self.get_src_16(src);
        self.write_dst_16(src, val + 1);
        5
    }

    // Decrement Register Pair
    fn dcx(&mut self, src: u8) -> usize {
        let src = match src {
            0 => Operand16::RegPairB,
            1 => Operand16::RegPairD,
            2 => Operand16::RegPairH,
            3 => Operand16::SP,
            _ => { panic!("Invalid src passed to DCX: {}!", src); }
        };
        
        let val = self.get_src_16(src);
        self.write_dst_16(src, val - 1);
        5
    }

    // Exchange Registers
    fn xchg(&mut self) -> usize {
        let temp = self.registers.pair_h();
        self.registers.set_pair_h(self.registers.pair_d());
        self.registers.set_pair_d(temp);
        4
    }

    // Exchange Stack
    fn xthl(&mut self, memory: &mut impl MemoryAccess) -> usize {
        let temp = self.registers.h();
        self.registers.set_h(memory.read(self.registers.sp() + 1));
        memory.write(self.registers.sp() + 1, temp);
        
        let temp = self.registers.l();
        self.registers.set_l(memory.read(self.registers.sp()));
        memory.write(self.registers.sp(), temp);
        18
    }

    // Load SP From H and L
    fn sphl(&mut self) -> usize {
        self.registers.set_sp(self.registers.pair_h());
        5
    }

    // Load Register Pair Immediate
    fn lxi(&mut self, dst: u8, memory: &impl MemoryAccess) -> usize {
        let dst = match dst {
            0 => Operand16::RegPairB,
            1 => Operand16::RegPairD,
            2 => Operand16::RegPairH,
            3 => Operand16::SP,
            _ => { panic!("Invalid dst passed to LXI: {}!", dst); }
        };

        self.load_imm16(memory);
        let val = self.registers.pair_w();
        self.write_dst_16(dst, val);
        10
    }

    // Store Accumulator Direct
    fn sta(&mut self, memory: &mut impl MemoryAccess) -> usize {
        self.load_imm16(memory);
        memory.write(self.registers.pair_w(), self.registers.accumulator());
        13
    }

    // Load Accumulator Direct
    fn lda(&mut self, memory: &mut impl MemoryAccess) -> usize {
        self.load_imm16(memory);
        self.registers.set_accumulator(memory.read(self.registers.pair_w()));
        13
    }

    // Store H and L Direct
    fn shld(&mut self, memory: &mut impl MemoryAccess) -> usize {
        self.load_imm16(memory);
        memory.write(self.registers.pair_w(), self.registers.l());
        memory.write(self.registers.pair_w() + 1, self.registers.h());
        16
    }

    // Load H and L Direct
    fn lhld(&mut self, memory: &mut impl MemoryAccess) -> usize {
        self.load_imm16(memory);
        self.registers.set_l(memory.read(self.registers.pair_w()));
        self.registers.set_h(memory.read(self.registers.pair_w()));
        15
    }

    // Load Program Counter
    fn pchl(&mut self) -> usize {
        self.registers.set_pc(self.registers.pair_h());
        5
    }

    fn check_condition(&self, condition_code: u8) -> bool {
        match condition_code {
            0 => !self.registers.status_zero(),
            1 => self.registers.status_zero(),
            2 => !self.registers.status_carry(),
            3 => self.registers.status_carry(),
            4 => !self.registers.status_parity(),
            5 => self.registers.status_parity(),
            6 => !self.registers.status_sign(),
            7 => self.registers.status_sign(),
            _ => true
        }
    }

    // Jump
    fn jmp(&mut self, condition: u8, memory: &impl MemoryAccess) -> usize {
        self.load_imm16(memory);

        if self.check_condition(condition) {
            self.registers.set_pc(self.registers.pair_w());
        }

        return 10
    }

    // Call
    fn call(&mut self, condition: u8, memory: &mut impl MemoryAccess) -> usize {
        self.load_imm16(memory);

        if self.check_condition(condition) {
            self.push_stack((self.registers.pc() >> 8) as u8, memory);
            self.push_stack((self.registers.pc() & 0xFF) as u8, memory);
            self.registers.set_pc(self.registers.pair_w());
            return 17;
        }
        return 11
    }
    
    // Return
    fn ret(&mut self, condition: u8, memory: &mut impl MemoryAccess) -> usize {
        if true {
            let new_pc:u16 = make_u16(self.pop_stack(memory),
                                      self.pop_stack(memory));
            self.registers.set_pc(new_pc);
            return if condition > 7 {10} else {11};
        };
        5
    }
    fn rst(&mut self, exp: u8, memory: &mut impl MemoryAccess) -> usize {
        self.push_stack((self.registers.pc() >> 8) as u8, memory);
        self.push_stack((self.registers.pc() & 0xFF) as u8, memory);
        self.registers.set_pc((exp as u16) << 3);
        11
    }
    // Enable Interrupts
    fn ei(&mut self) -> usize {
        self.inte = true;
        4
    }

    // Disable Interrupts
    fn di(&mut self) -> usize {
        self.inte = false;
        4
    }

    // Input
    fn input(&mut self) -> usize {
        let port = self.registers.z();

        if self.port_callbacks[port as usize].is_some() {
            let callback = self.port_callbacks[port as usize].unwrap().input_callback;
            self.registers.set_accumulator(callback());
        }

        10
    }

    // Output
    fn out(&self) -> usize {
        let port = self.registers.z();

        if self.port_callbacks[port as usize].is_some() {
            let callback = self.port_callbacks[port as usize].unwrap().output_callback;
            callback(self.registers.accumulator());
        }

        10
    }

    // Halt
    fn hlt(&mut self) -> usize {
        self.stopped = true;
        7
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory::Memory as Memory;

    #[test]
    fn test_cma() {
        let mut memory: Memory<1> = Memory::new();
        memory.write(0, Instruction::CMA as u8);

        let mut cpu = Intel8080::new();
        cpu.registers.set_pc(0);
        cpu.registers.set_accumulator(0b01010001);

        cpu.step(&mut memory);

        assert_eq!(cpu.registers.accumulator(), 0b10101110);
    }

    #[test]
    fn test_daa() {
        let mut memory: Memory<1> = Memory::new();
        memory.write(0, Instruction::DAA as u8);

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
        memory.write(0, Instruction::MOV_M_A as u8);
        memory.write(1, Instruction::MOV_B_M as u8);
        memory.write(2, Instruction::MOV_C_B as u8);

        let mut cpu = Intel8080::new();
        cpu.registers.set_pc(0);
        cpu.registers.set_pair_h(160);
        cpu.registers.set_accumulator(0x42);

        cpu.step(&mut memory);

        assert_eq!(memory.read(160u16), cpu.registers.accumulator());

        cpu.step(&mut memory);

        assert_eq!(memory.read(160), cpu.registers.b());

        cpu.step(&mut memory);

        assert_eq!(cpu.registers.b(), cpu.registers.c());
    }

    #[test]
    fn test_inr() {
        let mut memory: Memory<1> = Memory::new();
        memory.write(0, Instruction::INR_C as u8);

        let mut cpu = Intel8080::new();
        cpu.registers.set_pc(0);
        cpu.registers.set_c(0x99);

        cpu.step(&mut memory);

        assert_eq!(cpu.registers.c(), 0x9A);
    }
    
    #[test]
    fn test_dcr() {
        let mut memory: Memory<200> = Memory::new();
        memory.write(0, Instruction::DCR_M as u8);
        memory.write(0x50, 0x40);

        let mut cpu = Intel8080::new();
        cpu.registers.set_pc(0);
        cpu.registers.set_h(0x00);
        cpu.registers.set_l(0x50);
        

        cpu.step(&mut memory);

        assert_eq!(memory.read(0x50), 0x3F);
    }

    #[test]
    fn test_cmp() {
        let mut memory: Memory<40> = Memory::new();
        memory.write(0, Instruction::CMP_E as u8);
        memory.write(1, Instruction::CMP_H as u8);
        memory.write(2, Instruction::CMP_B as u8);

        let mut cpu = Intel8080::new();
        cpu.registers.set_pc(0);
        cpu.registers.set_accumulator(0x0A);
        cpu.registers.set_e(0x05);

        cpu.step(&mut memory);

        assert_eq!(cpu.registers.accumulator(), 0x0A);
        assert!(cpu.registers.status_carry());


        cpu.registers.set_accumulator(0x02);
        cpu.registers.set_h(0x05);

        cpu.step(&mut memory);

        assert_eq!(cpu.registers.accumulator(), 0x02);
        assert!(!cpu.registers.status_carry());
        assert!(!cpu.registers.status_zero());


        cpu.registers.set_accumulator(0x05);
        cpu.registers.set_b(0x05);

        cpu.step(&mut memory);

        assert_eq!(cpu.registers.accumulator(), 0x05);
        assert!(cpu.registers.status_carry());
        assert!(cpu.registers.status_zero());
    }

    #[test]
    fn test_rlc() {
        let mut memory: Memory<40> = Memory::new();
        memory.write(0, Instruction::RLC as u8);

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
        memory.write(0, Instruction::RRC as u8);

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
        memory.write(0, Instruction::RAL as u8);

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
        memory.write(0, Instruction::RAR as u8);

        let mut cpu = Intel8080::new();
        cpu.registers.set_pc(0);
        cpu.registers.set_accumulator(0b01101010);
        cpu.registers.set_status_carry(true);

        cpu.step(&mut memory);

        assert_eq!(cpu.registers.accumulator(), 0b10110101);
        assert!(!cpu.registers.status_carry());
    }

    #[test]
    fn test_adi() {
        let mut memory: Memory<40> = Memory::new();
        memory.write(0, Instruction::ADI as u8);
        memory.write(1, 0x42);

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
        let mut memory: Memory<40> = Memory::new();
        memory.write(0, Instruction::ACI as u8);
        memory.write(1, 0x42);

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
    fn test_xthl() {
        let mut memory: Memory<40> = Memory::new();
        memory.write(0, Instruction::XTHL as u8);
        memory.write(20, 0xFF);
        memory.write(21, 0xF0);
        memory.write(22, 0x0D);
        memory.write(23, 0xFF);

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

        assert_eq!(memory.read(20), 0xFF);
        assert_eq!(memory.read(21), 0x3C);
        assert_eq!(memory.read(22), 0x0B);
        assert_eq!(memory.read(23), 0xFF);
    }
}
