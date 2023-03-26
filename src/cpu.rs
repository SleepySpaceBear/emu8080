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
    Immediate = 4
}

impl From<u8> for Operand16 {
    fn from(orig: u8) -> Self {
        match orig {
            0 => return Operand16::RegPairB,
            1 => return Operand16::RegPairD,
            2 => return Operand16::RegPairH,
            3 => return Operand16::PSW,
            _ => return Operand16::Immediate
        }
    }
}

struct Intel8080 {
    registers: Registers
}

impl Intel8080 {
    fn init(&mut self) {
    
    }

    fn fetch_instruction(&mut self, memory: &Vec<u8>) -> Instruction {
        let instruction: Instruction = Instruction::from(memory[self.registers.pc() as usize]);   
        self.registers.set_pc(self.registers.pc() + 1);
        return instruction
    }

    fn do_instruction(&mut self, memory: &mut Vec<u8>) {
        let instruction = self.fetch_instruction(memory);

        match instruction {
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
            Instruction::DCR_C => { self.dcr(Operand8::RegC, memory)  },
            Instruction::DCR_D => { self.dcr(Operand8::RegD, memory) },
            Instruction::DCR_E => { self.dcr(Operand8::RegE, memory) },
            Instruction::DCR_H => { self.dcr(Operand8::RegH, memory) },
            Instruction::DCR_L => { self.dcr(Operand8::RegL, memory) },
            Instruction::DCR_M => { self.dcr(Operand8::Memory, memory) },
            Instruction::DCR_A => { self.dcr(Operand8::RegA, memory) },
            Instruction::CMA => { self.cma() },
            Instruction::DAA => { self.daa() },
            Instruction::MOV_B_C => { self.mov(Operand8::RegB, Operand8::RegC,  memory) },
            Instruction::MOV_B_D => { self.mov(Operand8::RegB, Operand8::RegD,  memory) },
            Instruction::MOV_B_E => { self.mov(Operand8::RegB, Operand8::RegE,  memory) },
            Instruction::MOV_B_H => { self.mov(Operand8::RegB, Operand8::RegH,  memory) },
            Instruction::MOV_B_L => { self.mov(Operand8::RegB, Operand8::RegL,  memory) },
            Instruction::MOV_B_M => { self.mov(Operand8::RegB, Operand8::Memory, memory) },
            Instruction::MOV_B_A => { self.mov(Operand8::RegB, Operand8::RegA,  memory) },
            Instruction::MOV_C_B => { self.mov(Operand8::RegC, Operand8::RegB,  memory) },
            Instruction::MOV_C_D => { self.mov(Operand8::RegC, Operand8::RegD,  memory) },
            Instruction::MOV_C_E => { self.mov(Operand8::RegC, Operand8::RegE,  memory) },
            Instruction::MOV_C_H => { self.mov(Operand8::RegC, Operand8::RegH,  memory) },
            Instruction::MOV_C_L => { self.mov(Operand8::RegC, Operand8::RegL,  memory) },
            Instruction::MOV_C_M => { self.mov(Operand8::RegC, Operand8::Memory, memory) },
            Instruction::MOV_C_A => { self.mov(Operand8::RegC, Operand8::RegA,  memory) },
            Instruction::MOV_D_B => { self.mov(Operand8::RegD, Operand8::RegB,  memory) },
            Instruction::MOV_D_C => { self.mov(Operand8::RegD, Operand8::RegC,  memory) },
            Instruction::MOV_D_E => { self.mov(Operand8::RegD, Operand8::RegE,  memory) },
            Instruction::MOV_D_H => { self.mov(Operand8::RegD, Operand8::RegH,  memory) },
            Instruction::MOV_D_L => { self.mov(Operand8::RegD, Operand8::RegL,  memory) },
            Instruction::MOV_D_M => { self.mov(Operand8::RegD, Operand8::Memory, memory) },
            Instruction::MOV_D_A => { self.mov(Operand8::RegD, Operand8::RegA,  memory) },
            Instruction::MOV_E_B => { self.mov(Operand8::RegE, Operand8::RegB,  memory) },
            Instruction::MOV_E_C => { self.mov(Operand8::RegE, Operand8::RegC,  memory) },
            Instruction::MOV_E_D => { self.mov(Operand8::RegE, Operand8::RegD,  memory) },
            Instruction::MOV_E_H => { self.mov(Operand8::RegE, Operand8::RegH,  memory) },
            Instruction::MOV_E_L => { self.mov(Operand8::RegE, Operand8::RegL,  memory) },
            Instruction::MOV_E_M => { self.mov(Operand8::RegE, Operand8::Memory, memory) },
            Instruction::MOV_E_A => { self.mov(Operand8::RegE, Operand8::RegA,  memory) },
            Instruction::MOV_H_B => { self.mov(Operand8::RegH, Operand8::RegB,  memory) },
            Instruction::MOV_H_C => { self.mov(Operand8::RegH, Operand8::RegC,  memory) },
            Instruction::MOV_H_D => { self.mov(Operand8::RegH, Operand8::RegD,  memory) },
            Instruction::MOV_H_E => { self.mov(Operand8::RegE, Operand8::RegE,  memory) },
            Instruction::MOV_H_L => { self.mov(Operand8::RegH, Operand8::RegL,  memory) },
            Instruction::MOV_H_M => { self.mov(Operand8::RegH, Operand8::Memory, memory) },
            Instruction::MOV_H_A => { self.mov(Operand8::RegH, Operand8::RegA,  memory) },
            Instruction::MOV_L_B => { self.mov(Operand8::RegL, Operand8::RegB,  memory) },
            Instruction::MOV_L_C => { self.mov(Operand8::RegL, Operand8::RegC,  memory) },
            Instruction::MOV_L_D => { self.mov(Operand8::RegL, Operand8::RegD,  memory) },
            Instruction::MOV_L_E => { self.mov(Operand8::RegL, Operand8::RegE,  memory) },
            Instruction::MOV_L_H => { self.mov(Operand8::RegL, Operand8::RegH,  memory) },
            Instruction::MOV_L_M => { self.mov(Operand8::RegL, Operand8::Memory, memory) },
            Instruction::MOV_L_A => { self.mov(Operand8::RegL, Operand8::RegA,  memory) },
            Instruction::MOV_M_B => { self.mov(Operand8::Memory, Operand8::RegB, memory) },
            Instruction::MOV_M_C => { self.mov(Operand8::Memory, Operand8::RegC, memory) },
            Instruction::MOV_M_D => { self.mov(Operand8::Memory, Operand8::RegD, memory) },
            Instruction::MOV_M_E => { self.mov(Operand8::Memory, Operand8::RegE, memory) },
            Instruction::MOV_M_H => { self.mov(Operand8::Memory, Operand8::RegH, memory) },
            Instruction::MOV_M_L => { self.mov(Operand8::Memory, Operand8::RegL, memory) },
            Instruction::MOV_M_A => { self.mov(Operand8::Memory, Operand8::RegH, memory) },
            Instruction::MOV_A_B => { self.mov(Operand8::RegA, Operand8::RegB,  memory) },
            Instruction::MOV_A_C => { self.mov(Operand8::RegA, Operand8::RegC,  memory) },
            Instruction::MOV_A_D => { self.mov(Operand8::RegA, Operand8::RegD,  memory) },
            Instruction::MOV_A_E => { self.mov(Operand8::RegA, Operand8::RegE,  memory) },
            Instruction::MOV_A_H => { self.mov(Operand8::RegA, Operand8::RegH,  memory) },
            Instruction::MOV_A_L => { self.mov(Operand8::RegA, Operand8::RegL,  memory) },
            Instruction::MOV_A_M => { self.mov(Operand8::RegA, Operand8::Memory, memory) },
            Instruction::STAX_B => { self.stax(Operand16::RegPairB, memory) },
            Instruction::STAX_D => { self.stax(Operand16::RegPairD, memory) },
            Instruction::LDAX_B => { self.ldax(Operand16::RegPairB, memory) },
            Instruction::LDAX_D => { self.ldax(Operand16::RegPairD, memory)},
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
            Instruction::SBB_A => { self.sbb(Operand8::RegA, memory)  },
            Instruction::ANA_B => { self.ana(Operand8::RegB, memory) },
            Instruction::ANA_C => { self.ana(Operand8::RegC, memory) },
            Instruction::ANA_D => { self.ana(Operand8::RegD, memory) },
            Instruction::ANA_E => { self.ana(Operand8::RegE, memory) },
            Instruction::ANA_H => { self.ana(Operand8::RegH, memory) },
            Instruction::ANA_L => { self.ana(Operand8::RegL, memory)} ,
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
            Instruction::DAD_B => { },
            Instruction::DAD_D => { },
            Instruction::DAD_H => { },
            Instruction::DAD_SP => { },
            Instruction::INX_B => { },
            Instruction::INX_D => { },
            Instruction::INX_H => { },
            Instruction::INX_SP => { },
            Instruction::DCX_B => { },
            Instruction::DCX_D => { },
            Instruction::DCX_H => { },
            Instruction::DCX_SP => { },
            Instruction::XCHG => { },
            Instruction::XTHL => { },
            Instruction::SPHL => { },
            Instruction::LXI_B => { },
            Instruction::LXI_D => { },
            Instruction::LXI_H => { },
            Instruction::LXI_SP => { },
            Instruction::MVI_B => { self.fetch_instruction(memory); self.mov(Operand8::RegB, Operand8::Immediate, memory) },
            Instruction::MVI_C => { self.fetch_instruction(memory); self.mov(Operand8::RegC, Operand8::Immediate, memory) },
            Instruction::MVI_D => { self.fetch_instruction(memory); self.mov(Operand8::RegD, Operand8::Immediate, memory) },
            Instruction::MVI_E => { self.fetch_instruction(memory); self.mov(Operand8::RegE, Operand8::Immediate, memory) },
            Instruction::MVI_H => { self.fetch_instruction(memory); self.mov(Operand8::RegH, Operand8::Immediate, memory) },
            Instruction::MVI_L => { self.fetch_instruction(memory); self.mov(Operand8::RegL, Operand8::Immediate, memory) },
            Instruction::MVI_M => { self.fetch_instruction(memory); self.mov(Operand8::Memory, Operand8::Immediate, memory) },
            Instruction::MVI_A => { self.fetch_instruction(memory); self.mov(Operand8::RegA, Operand8::Immediate, memory) },
            Instruction::ADI => { },
            Instruction::ACI => { },
            Instruction::SUI => { },
            Instruction::SBI => { },
            Instruction::ANI => { },
            Instruction::XRI => { },
            Instruction::ORI => { },
            Instruction::CPI => { },
            Instruction::STA => { },
            Instruction::LDA => { },
            Instruction::SHLD => { },
            Instruction::LHLD => { },
            Instruction::PCHL => { },
            Instruction::JMP => { },
            Instruction::JC => { },
            Instruction::JNC => { },
            Instruction::JZ => { },
            Instruction::JNZ => { },
            Instruction::JM => { },
            Instruction::JP => { },
            Instruction::JPE => { },
            Instruction::JPO => { },
            Instruction::CALL => { },
            Instruction::CC => { },
            Instruction::CNC => { },
            Instruction::CZ => { },
            Instruction::CNZ => { },
            Instruction::CM => { },
            Instruction::CP => { },
            Instruction::CPE => { },
            Instruction::CPO => { },
            Instruction::RET => { },
            Instruction::RC => { },
            Instruction::RNC => { },
            Instruction::RZ => { },
            Instruction::RNZ => { },
            Instruction::RM => { },
            Instruction::RP => { },
            Instruction::RPE => { },
            Instruction::RPO => { },
            Instruction::RST_1 => { },
            Instruction::RST_2 => { },
            Instruction::RST_3 => { },
            Instruction::RST_4 => { },
            Instruction::RST_5 => { },
            Instruction::RST_6 => { },
            Instruction::RST_7 => { },
            Instruction::RST_8 => { },
            Instruction::EI => { },
            Instruction::DI => { },
            Instruction::IN => { },
            Instruction::OUT => { },
            Instruction::HLT => { },
            _ => { }
        }
    }
 
    fn get_src(&mut self, src: Operand8, memory: &Vec<u8>) -> u8 {
        match src {
            Operand8::RegB => { self.registers.b() },
            Operand8::RegC => { self.registers.c() },
            Operand8::RegD => { self.registers.d() },
            Operand8::RegE => { self.registers.e() },
            Operand8::RegH => { self.registers.h() },
            Operand8::RegL => { self.registers.l() },
            Operand8::Memory => { memory[self.registers.pair_h() as usize] }
            Operand8::RegA => { self.registers.accumulator() },
            Operand8::Immediate => { 
                memory[(self.registers.pc() - 1) as usize]
            }
        }
    }

    fn write_dst(&mut self, dst: Operand8, val: u8, memory: &mut Vec<u8>) {
        match dst {
            Operand8::RegB => { self.registers.set_b(val) },
            Operand8::RegC => { self.registers.set_c(val) },
            Operand8::RegD => { self.registers.set_d(val) },
            Operand8::RegE => { self.registers.set_e(val) },
            Operand8::RegH => { self.registers.set_h(val) },
            Operand8::RegL => { self.registers.set_l(val) },
            Operand8::Memory => { memory[self.registers.pair_h() as usize] = val; },
            Operand8::RegA => { self.registers.set_accumulator(val) },
            Operand8::Immediate => {} 
        }
    }

    fn get_src_16(&mut self, src: Operand16, memory: &Vec<u8>) -> u16 {
        match src {
            Operand16::PSW => { self.registers.psw() },
            Operand16::RegPairB => { self.registers.pair_b() },
            Operand16::RegPairD => { self.registers.pair_d() },
            Operand16::RegPairH => { self.registers.pair_h() },
            Operand16::Immediate => { 
                make_u16(memory[(self.registers.pc() - 1) as usize], 
                         memory[(self.registers.pc() - 2) as usize])
            }
        }
    }

    fn write_dst_16(&mut self, src: Operand16, val: u16) {
        match src {
            Operand16::PSW => { self.registers.set_psw(val) },
            Operand16::RegPairB => { self.registers.set_pair_b(val) },
            Operand16::RegPairD => { self.registers.set_pair_d(val) },
            Operand16::RegPairH => { self.registers.set_pair_h(val) }
            Operand16::Immediate => {}
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
    fn inr(&mut self, reg: Operand8, memory: &mut Vec<u8>) {
        let val = self.get_src(reg, memory);
        let val = val.wrapping_add(1);
        self.write_dst(reg, val, memory);
        self.set_condition(val, val == 0, val % 0x10 == 0);
    }

    // Decrement Register or Memory
    fn dcr(&mut self, reg: Operand8, memory: &mut Vec<u8>) {
        // Note: This is equivalent in Intel8080 to adding 0xFF to the value
        // due to the usage of 2s complement representation. Carry bits will be 
        // set accordingly.
        let orig_val = self.get_src(reg, memory);
        let new_val = orig_val.wrapping_sub(1);
        self.write_dst(reg, new_val, memory);
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
    fn mov(&mut self, src: Operand8, dst: Operand8, memory: &mut Vec<u8>) {
        let src = self.get_src(src, memory);
        self.write_dst(dst, src, memory);
    }

    // Store Accumulator
    fn stax(&mut self, dst: Operand16, memory: &mut Vec<u8>) {
        match dst {
            Operand16::RegPairB => {
                memory[self.registers.pair_b() as usize] = self.registers.accumulator();
            },
            Operand16::RegPairD => {
                memory[self.registers.pair_d() as usize] = self.registers.accumulator();
            },
            _ => {}
        }
    }

    // Load Accumulator
    fn ldax(&mut self, dst: Operand16, memory: &mut Vec<u8>) {
        match dst {
            Operand16::RegPairB => {
                self.registers.set_accumulator(memory[self.registers.pair_b() as usize]);
            },
            Operand16::RegPairD => {
                self.registers.set_accumulator(memory[self.registers.pair_d() as usize]);
            },
            _ => {}
        }
    }

    // ADD Register or Memory to Accumulator
    fn add(&mut self, src: Operand8, memory: &mut Vec<u8>) {
        let old_val: u8 = self.registers.accumulator();
        let new_val: u8 = old_val.wrapping_add(self.get_src(src, memory)); 
        
        let carry: bool = check_carry(old_val, new_val);
        let aux_carry: bool = check_aux_carry(old_val, new_val);

        self.registers.set_accumulator(new_val as u8);
        self.set_condition(new_val as u8, carry, aux_carry)
    }

    // ADD Register or Memory to Accumulator With Carry
    fn adc(&mut self, src: Operand8, memory: &mut Vec<u8>) {
        let old_val: u8 = self.registers.accumulator();
        let new_val: u8 = old_val.wrapping_add(self.get_src(src, memory))
            .wrapping_add(self.registers.status_carry() as u8); 
        
        let carry: bool = check_carry(old_val, new_val);
        let aux_carry: bool = check_aux_carry(old_val, new_val);

        self.registers.set_accumulator(new_val);
        self.set_condition(new_val, carry, aux_carry)
    }

    // Subtract Register or Memory From Accumulator
    fn sub(&mut self, src: Operand8, memory: &mut Vec<u8>) {
        let old_val = self.registers.accumulator();
        let new_val = old_val.wrapping_add(twos_complement(self.get_src(src, memory)));

        let carry: bool = check_carry(old_val, new_val);
        let aux_carry: bool = check_aux_carry(old_val, new_val);

        self.registers.set_accumulator(new_val);
        self.set_condition(new_val, carry, aux_carry)
    } 

    // Subtract Register or Memory From Accumulator With Borrow
    fn sbb(&mut self, src: Operand8, memory: &mut Vec<u8>) {
        let old_val = self.registers.accumulator();
        let new_val = twos_complement(self.get_src(src, memory)
            .wrapping_add(self.registers.status_carry() as u8));
        let new_val = old_val.wrapping_add(new_val);

        let carry: bool = check_carry(old_val, new_val);
        let aux_carry: bool = check_aux_carry(old_val, new_val);

        self.registers.set_accumulator(new_val);
        self.set_condition(new_val, carry, aux_carry)
    }

    // Logical and Register or Memory With Accumulator
    fn ana(&mut self, src: Operand8, memory: &mut Vec<u8>) {
        let val: u8 = self.registers.accumulator() & self.get_src(src, memory);
        self.set_condition(val, false, false);
        self.registers.set_accumulator(val);
    }

    // Logical Exclusive-Or Register or Memory with Accumulator
    fn xra(&mut self, src: Operand8, memory: &mut Vec<u8>) {
        let val: u8 = self.registers.accumulator() ^ self.get_src(src, memory);
        self.set_condition(val, false, false);
        self.registers.set_accumulator(val);
    }

    // Logical or Register or Memory with Accumulator
    fn ora(&mut self, src: Operand8, memory: &mut Vec<u8>) {
        let val: u8 = self.registers.accumulator() | self.get_src(src, memory);
        self.set_condition(val, false, false);
        self.registers.set_accumulator(val);
    }

    // Compare Register or Memory with Accumulator
    fn cmp(&mut self, src:Operand8, memory: &mut Vec<u8>) {
        let old_val = self.registers.accumulator();
        let new_val = old_val.wrapping_add(twos_complement(self.get_src(src, memory)));

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
    fn push(&mut self, src: Operand16, memory: &mut Vec<u8>) {
        let first_register = match src {
            Operand16::RegPairB => { self.registers.b() },
            Operand16::RegPairD => { self.registers.d() },
            Operand16::RegPairH => { self.registers.h() },
            Operand16::PSW => { self.registers.status() },
            Operand16::Immediate => { 0 }
        };
        
        let second_register = match src {
            Operand16::RegPairB => { self.registers.c() },
            Operand16::RegPairD => { self.registers.e() },
            Operand16::RegPairH => { self.registers.l() },
            Operand16::PSW => { self.registers.accumulator() },
            Operand16::Immediate => { 0 }
        };

        memory[(self.registers.sp() - 1) as usize] = first_register;
        memory[(self.registers.sp() - 2) as usize] = second_register;
        self.registers.set_sp(self.registers.sp() - 2);
    }

    // Pop Data Off Stack
    fn pop(&mut self, dst: Operand16, memory: &Vec<u8>) {
        let first_register = memory[(self.registers.sp() + 1) as usize];
        let second_register = memory[(self.registers.sp()) as usize];

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
            _ => { }
        }

        self.registers.set_sp(self.registers.sp() + 2);
    }

    // Double Add
    fn dad(&mut self, src: Operand16) {

    }

    // Increment Register Pair
    fn inx(&mut self, src: Operand16) {

    }

    // Decrement Register Pair
    fn dcx(&mut self, src: Operand16) {

    }

    // Exchange Registers
    fn xchg(&mut self) {
        let temp = self.registers.pair_h();
        self.registers.set_pair_h(self.registers.pair_d());
        self.registers.set_pair_d(temp);
    }

    // Exchange Stack
    fn xthl(&mut self, memory: &mut Vec<u8>) {
        let temp = self.registers.l();
        self.registers.set_l(memory[self.registers.sp() as usize + 1]);
        memory[self.registers.sp() as usize + 1] = temp;
    }

    // Load SP From H and L
    fn sphl(&mut self) {
        self.registers.set_sp(self.registers.pair_h());
    }

    // Store Accumulator Direct
    fn sta(&mut self) {

    }

    // Load Accumulator Direct
    fn lda(&mut self) {

    }

    // Store H and L Direct
    fn shld(&mut self) {

    }

    // Load H and L Direct
    fn lhld(&mut self) {

    }

    // Load Program Counter
    fn pchl(&mut self) {

    }

    // Jump
    fn jmp(&mut self) {

    }

    // Jump If Carry
    fn jc(&mut self) {

    }

    // Jump No Carry
    fn jnc(&mut self) {

    }

    // Jump If Zero
    fn jz(&mut self) {

    }

    // Jump If Not Zero
    fn jnz(&mut self) {

    }

    // Jump If Minus
    fn jm(&mut self) {

    }

    // Jump If Positive
    fn jp(&mut self) {

    }

    // Jump If Parity Even
    fn jpe(&mut self) {

    }

    // Jump If Parity Odd
    fn jpo(&mut self) {

    }

    // Call
    fn call(&mut self) {

    }

    // Call If Carry
    fn cc(&mut self) {

    }

    // Call If No Carry
    fn cnc(&mut self) {

    }

    // Call If Zero
    fn cz(&mut self) {

    }

    // Call If Not Zero
    fn cnz(&mut self) {

    }

    // Call If Minus
    fn cm(&mut self) {

    }

    // Call If Plus
    fn cp(&mut self) {

    }

    // Call If Parity Even
    fn cpe(&mut self) {

    }

    // Call If Parity Odd
    fn cpo(&mut self) {

    }

    // Return
    fn ret(&mut self) {

    }

    // Return If Carry
    fn rc(&mut self) {

    }

    // Return If No Carry
    fn rnc(&mut self) {

    }

    // Return If Zero
    fn rz(&mut self) {

    }

    // Return If Not Zero
    fn rnz(&mut self) {

    }

    // Return If Minus
    fn rm(&mut self) {

    }

    // Return If Plus
    fn rp(&mut self) {

    }

    // Return If Parity Even
    fn rpe(&mut self) {

    }

    // Return If Parity Odd
    fn rpo(&mut self) {

    }

    // Restart
    fn rst_1(&mut self) {

    }

    // Restart
    fn rst_2(&mut self) {

    }

    // Restart
    fn rst_3(&mut self) {

    }

    // Restart
    fn rst_4(&mut self) {

    }

    // Restart
    fn rst_5(&mut self) {

    }

    // Restart
    fn rst_6(&mut self) {

    }

    // Restart
    fn rst_7(&mut self) {

    }

    // Restart
    fn rst_8(&mut self) {

    }

    // Enable Interrupts
    fn ei(&mut self) {

    }

    // Disable Interrupts
    fn di(&mut self) {

    }

    // Input
    fn input(&mut self) {

    }

    // Output
    fn out(&mut self) {

    }

    // Halt
    fn hlt(&mut self) {

    }

}
