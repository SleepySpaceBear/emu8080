use crate::registers::Registers as Registers;

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

struct Intel8080 {
    registers: Registers
}

impl Intel8080 {
    fn do_instruction(&mut self, instruction: u8) {
        let instruction: Instruction = Instruction::from(instruction);   
        match instruction {
            Instruction::NOP => { },
            Instruction::STC => { self.stc() },
            Instruction::CMC => { self.cmc() },
            Instruction::INR_B => { self.inr_b() },
            Instruction::INR_C => { self.inr_c() },
            Instruction::INR_D => { self.inr_d() },
            Instruction::INR_E => { self.inr_e() },
            Instruction::INR_H => { self.inr_h() },
            Instruction::INR_L => { self.inr_l() },
            Instruction::INR_M => { self.inr_m() },
            Instruction::INR_A => { self.inr_a() },
            Instruction::DCR_B => { self.dcr_b() },
            Instruction::DCR_C => { self.dcr_c() },
            Instruction::DCR_D => { self.dcr_d() },
            Instruction::DCR_E => { self.dcr_e() },
            Instruction::DCR_H => { self.dcr_h() },
            Instruction::DCR_L => { self.dcr_l() },
            Instruction::DCR_M => { self.dcr_m() },
            Instruction::DCR_A => { self.dcr_a() },
            Instruction::CMA => { self.cma() },
            Instruction::DAA => { self.daa() },
            Instruction::MOV_B_B => { },
            Instruction::MOV_B_C => { self.mov_b_c() },
            Instruction::MOV_B_D => { self.mov_b_d() },
            Instruction::MOV_B_E => { self.mov_b_e() },
            Instruction::MOV_B_H => { self.mov_b_h() },
            Instruction::MOV_B_L => { self.mov_b_l() },
            Instruction::MOV_B_M => { self.mov_b_m() },
            Instruction::MOV_B_A => { self.mov_b_a() },
            Instruction::MOV_C_B => { self.mov_c_b() },
            Instruction::MOV_C_C => { },
            Instruction::MOV_C_D => { self.mov_c_d() },
            Instruction::MOV_C_E => { self.mov_c_e() },
            Instruction::MOV_C_H => { self.mov_c_h() },
            Instruction::MOV_C_L => { self.mov_c_l() },
            Instruction::MOV_C_M => { self.mov_c_m() },
            Instruction::MOV_C_A => { self.mov_c_a() },
            Instruction::MOV_D_B => { self.mov_d_b() },
            Instruction::MOV_D_C => { self.mov_d_c() },
            Instruction::MOV_D_D => { },
            Instruction::MOV_D_E => { self.mov_d_e() },
            Instruction::MOV_D_H => { self.mov_d_h() },
            Instruction::MOV_D_L => { self.mov_d_l() },
            Instruction::MOV_D_M => { self.mov_d_m() },
            Instruction::MOV_D_A => { self.mov_d_a() },
            Instruction::MOV_E_B => { self.mov_e_b() },
            Instruction::MOV_E_C => { self.mov_e_c() },
            Instruction::MOV_E_D => { self.mov_e_d() },
            Instruction::MOV_E_E => { },
            Instruction::MOV_E_H => { self.mov_e_h() },
            Instruction::MOV_E_L => { self.mov_e_l() },
            Instruction::MOV_E_M => { self.mov_e_m() },
            Instruction::MOV_E_A => { self.mov_e_a() },
            Instruction::MOV_H_B => { self.mov_h_b() },
            Instruction::MOV_H_C => { self.mov_h_c() },
            Instruction::MOV_H_D => { self.mov_h_d() },
            Instruction::MOV_H_E => { self.mov_h_e() },
            Instruction::MOV_H_H => { },
            Instruction::MOV_H_L => { self.mov_h_l() },
            Instruction::MOV_H_M => { self.mov_h_m() },
            Instruction::MOV_H_A => { self.mov_h_a() },
            Instruction::MOV_L_B => { self.mov_l_b() },
            Instruction::MOV_L_C => { self.mov_l_c() },
            Instruction::MOV_L_D => { self.mov_l_d() },
            Instruction::MOV_L_E => { self.mov_l_e() },
            Instruction::MOV_L_H => { self.mov_l_h() },
            Instruction::MOV_L_L => { },
            Instruction::MOV_L_M => { self.mov_l_m() },
            Instruction::MOV_L_A => { self.mov_l_a() },
            Instruction::MOV_M_B => { self.mov_m_b() },
            Instruction::MOV_M_C => { self.mov_m_c() },
            Instruction::MOV_M_D => { self.mov_m_d() },
            Instruction::MOV_M_E => { self.mov_m_e() },
            Instruction::MOV_M_H => { self.mov_m_h() },
            Instruction::MOV_M_L => { self.mov_m_l() },
            Instruction::MOV_M_A => { self.mov_m_a() },
            Instruction::MOV_A_B => { self.mov_a_b() },
            Instruction::MOV_A_C => { self.mov_a_c() },
            Instruction::MOV_A_D => { self.mov_a_d() },
            Instruction::MOV_A_E => { self.mov_a_e() },
            Instruction::MOV_A_H => { self.mov_a_h() },
            Instruction::MOV_A_L => { self.mov_a_l() },
            Instruction::MOV_A_M => { self.mov_a_m() },
            Instruction::MOV_A_A => { },
            Instruction::STAX_B => { self.stax_b() },
            Instruction::STAX_D => { self.stax_d() },
            Instruction::LDAX_B => { self.ldax_b() },
            Instruction::LDAX_D => { self.ldax_d() },
            Instruction::ADD_B => { self.add_b() },
            Instruction::ADD_C => { self.add_c() },
            Instruction::ADD_D => { self.add_d() },
            Instruction::ADD_E => { self.add_e() },
            Instruction::ADD_H => { self.add_h() },
            Instruction::ADD_L => { self.add_l() },
            Instruction::ADD_M => { self.add_m() },
            Instruction::ADD_A => { self.add_a() },
            Instruction::ADC_B => { self.adc_b() },
            Instruction::ADC_C => { self.adc_c() },
            Instruction::ADC_D => { self.adc_d() },
            Instruction::ADC_E => { self.adc_e() },
            Instruction::ADC_H => { self.adc_h() },
            Instruction::ADC_L => { self.adc_l() },
            Instruction::ADC_M => { self.adc_m() },
            Instruction::ADC_A => { self.adc_a() },
            Instruction::SUB_B => { self.sub_b() },
            Instruction::SUB_C => { self.sub_c() },
            Instruction::SUB_D => { self.sub_d() },
            Instruction::SUB_E => { self.sub_e() },
            Instruction::SUB_H => { self.sub_h() },
            Instruction::SUB_L => { self.sub_l() },
            Instruction::SUB_M => { self.sub_m() },
            Instruction::SUB_A => { self.sub_a() },
            Instruction::SBB_B => { self.sbb_b() },
            Instruction::SBB_C => { self.sbb_c() },
            Instruction::SBB_D => { self.sbb_d() },
            Instruction::SBB_E => { self.sbb_e() },
            Instruction::SBB_H => { self.sbb_h() },
            Instruction::SBB_L => { self.sbb_l() },
            Instruction::SBB_M => { self.sbb_m() },
            Instruction::SBB_A => { self.sbb_a() },
            Instruction::ANA_B => { self.ana_b() },
            Instruction::ANA_C => { self.ana_c() },
            Instruction::ANA_D => { self.ana_d() },
            Instruction::ANA_E => { self.ana_e() },
            Instruction::ANA_H => { self.ana_h() },
            Instruction::ANA_L => { self.ana_l() },
            Instruction::ANA_M => { self.ana_m() },
            Instruction::ANA_A => { self.ana_a() },
            Instruction::XRA_B => { self.xra_b() },
            Instruction::XRA_C => { self.xra_c() },
            Instruction::XRA_D => { self.xra_d() },
            Instruction::XRA_E => { self.xra_e() },
            Instruction::XRA_H => { self.xra_h() },
            Instruction::XRA_L => { self.xra_l() },
            Instruction::XRA_M => { self.xra_m() },
            Instruction::XRA_A => { self.xra_a() },
            Instruction::ORA_B => { self.ora_b() },
            Instruction::ORA_C => { self.ora_c() },
            Instruction::ORA_D => { self.ora_d() },
            Instruction::ORA_E => { self.ora_e() },
            Instruction::ORA_H => { self.ora_h() },
            Instruction::ORA_L => { self.ora_l() },
            Instruction::ORA_M => { self.ora_m() },
            Instruction::ORA_A => { self.ora_a() },
            Instruction::CMP_B => { self.cmp_b() },
            Instruction::CMP_C => { self.cmp_c() },
            Instruction::CMP_D => { self.cmp_d() },
            Instruction::CMP_E => { self.cmp_e() },
            Instruction::CMP_H => { self.cmp_h() },
            Instruction::CMP_L => { self.cmp_l() },
            Instruction::CMP_M => { self.cmp_m() },
            Instruction::CMP_A => { self.cmp_a() },
            Instruction::RLC => { self.rlc() },
            Instruction::RRC => { self.rrc() },
            Instruction::RAL => { self.ral() },
            Instruction::RAR => { self.rar() },
            Instruction::PUSH_B => { self.push_b() },
            Instruction::PUSH_D => { self.push_d() },
            Instruction::PUSH_H => { self.push_h() },
            Instruction::PUSH_PSW => { self.push_psw() },
            Instruction::POP_B => { self.pop_b() },
            Instruction::POP_D => { self.pop_d() },
            Instruction::POP_H => { self.pop_h() },
            Instruction::POP_PSW => { self.pop_psw() },
            Instruction::DAD_B => { self.dad_b() },
            Instruction::DAD_D => { self.dad_d() },
            Instruction::DAD_H => { self.dad_h() },
            Instruction::DAD_SP => { self.dad_sp() },
            Instruction::INX_B => { self.inx_b() },
            Instruction::INX_D => { self.inx_d() },
            Instruction::INX_H => { self.inx_h() },
            Instruction::INX_SP => { self.inx_sp() },
            Instruction::DCX_B => { self.dcx_b() },
            Instruction::DCX_D => { self.dcx_d() },
            Instruction::DCX_H => { self.dcx_h() },
            Instruction::DCX_SP => { self.dcx_sp() },
            Instruction::XCHG => { self.xchg() },
            Instruction::XTHL => { self.xthl() },
            Instruction::SPHL => { self.sphl() },
            Instruction::LXI_B => { self.lxi_b() },
            Instruction::LXI_D => { self.lxi_d() },
            Instruction::LXI_H => { self.lxi_h() },
            Instruction::LXI_SP => { self.lxi_sp() },
            Instruction::MVI_B => { self.mvi_b() },
            Instruction::MVI_C => { self.mvi_c() },
            Instruction::MVI_D => { self.mvi_d() },
            Instruction::MVI_E => { self.mvi_e() },
            Instruction::MVI_H => { self.mvi_h() },
            Instruction::MVI_L => { self.mvi_l() },
            Instruction::MVI_M => { self.mvi_m() },
            Instruction::MVI_A => { self.mvi_a() },
            Instruction::ADI => { self.adi() },
            Instruction::ACI => { self.aci() },
            Instruction::SUI => { self.sui() },
            Instruction::SBI => { self.sbi() },
            Instruction::ANI => { self.ani() },
            Instruction::XRI => { self.xri() },
            Instruction::ORI => { self.ori() },
            Instruction::CPI => { self.cpi() },
            Instruction::STA => { self.sta() },
            Instruction::LDA => { self.lda() },
            Instruction::SHLD => { self.shld() },
            Instruction::LHLD => { self.lhld() },
            Instruction::PCHL => { self.pchl() },
            Instruction::JMP => { self.jmp() },
            Instruction::JC => { self.jc() },
            Instruction::JNC => { self.jnc() },
            Instruction::JZ => { self.jz() },
            Instruction::JNZ => { self.jnz() },
            Instruction::JM => { self.jm() },
            Instruction::JP => { self.jp() },
            Instruction::JPE => { self.jpe() },
            Instruction::JPO => { self.jpo() },
            Instruction::CALL => { self.call() },
            Instruction::CC => { self.cc() },
            Instruction::CNC => { self.cnc() },
            Instruction::CZ => { self.cz() },
            Instruction::CNZ => { self.cnz() },
            Instruction::CM => { self.cm() },
            Instruction::CP => { self.cp() },
            Instruction::CPE => { self.cpe() },
            Instruction::CPO => { self.cpo() },
            Instruction::RET => { self.ret() },
            Instruction::RC => { self.rc() },
            Instruction::RNC => { self.rnc() },
            Instruction::RZ => { self.rz() },
            Instruction::RNZ => { self.rnz() },
            Instruction::RM => { self.rm() },
            Instruction::RP => { self.rp() },
            Instruction::RPE => { self.rpe() },
            Instruction::RPO => { self.rpo() },
            Instruction::RST_1 => { self.rst_1() },
            Instruction::RST_2 => { self.rst_2() },
            Instruction::RST_3 => { self.rst_3() },
            Instruction::RST_4 => { self.rst_4() },
            Instruction::RST_5 => { self.rst_5() },
            Instruction::RST_6 => { self.rst_6() },
            Instruction::RST_7 => { self.rst_7() },
            Instruction::RST_8 => { self.rst_8() },
            Instruction::EI => { self.ei() },
            Instruction::DI => { self.di() },
            Instruction::IN => { self.input() },
            Instruction::OUT => { self.out() },
            Instruction::HLT => { self.hlt() },
            _ => { }
        }
    }
 
    // Set Carry
    fn stc(&mut self) {

    }

    // Complement Carry
    fn cmc(&mut self) {

    }

    // Increment Register or Memory 
    fn inr_b(&mut self) {
        self.registers.set_b(self.registers.b().wrapping_add(1));
    }

    // Increment Register or Memory
    fn inr_c(&mut self) {
        self.registers.set_c(self.registers.c().wrapping_add(1));
    }

    // Increment Register or Memory
    fn inr_d(&mut self) {

    }

    // Increment Register or Memory
    fn inr_e(&mut self) {

    }

    // Increment Register or Memory
    fn inr_h(&mut self) {

    }

    // Increment Register or Memory
    fn inr_l(&mut self) {

    }

    // Increment Register or Memory
    fn inr_m(&mut self) {

    }

    // Increment Register or Memory
    fn inr_a(&mut self) {

    }

    // Decrement Register or Memory 
    fn dcr_b(&mut self) {

    }

    // Decrement Register or Memory
    fn dcr_c(&mut self) {

    }

    // Decrement Register or Memory
    fn dcr_d(&mut self) {

    }

    // Decrement Register or Memory
    fn dcr_e(&mut self) {

    }

    // Decrement Register or Memory
    fn dcr_h(&mut self) {

    }

    // Decrement Register or Memory
    fn dcr_l(&mut self) {

    }

    // Decrement Register or Memory
    fn dcr_m(&mut self) {

    }

    // Decrement Register or Memory
    fn dcr_a(&mut self) {

    }

    // Complement Accumulator
    fn cma(&mut self) {

    }

    // Decimal Adjust Accumulator
    fn daa(&mut self) {

    }

    // Move
    fn mov_b_c(&mut self) {

    }

    // Move
    fn mov_b_d(&mut self) {

    }

    // Move
    fn mov_b_e(&mut self) {

    }

    // Move
    fn mov_b_h(&mut self) {

    }

    // Move
    fn mov_b_l(&mut self) {

    }

    // Move
    fn mov_b_m(&mut self) {

    }

    // Move
    fn mov_b_a(&mut self) {

    }

    // Move
    fn mov_c_b(&mut self) {

    }

    // Move
    fn mov_c_d(&mut self) {

    }

    // Move
    fn mov_c_e(&mut self) {

    }

    // Move
    fn mov_c_h(&mut self) {

    }

    // Move
    fn mov_c_l(&mut self) {

    }

    // Move
    fn mov_c_m(&mut self) {

    }

    // Move
    fn mov_c_a(&mut self) {

    }

    // Move
    fn mov_d_b(&mut self) {

    }

    // Move
    fn mov_d_c(&mut self) {

    }

    // Move
    fn mov_d_e(&mut self) {

    }

    // Move
    fn mov_d_h(&mut self) {

    }

    // Move
    fn mov_d_l(&mut self) {

    }

    // Move
    fn mov_d_m(&mut self) {

    }

    // Move
    fn mov_d_a(&mut self) {

    }

    // Move
    fn mov_e_b(&mut self) {

    }

    // Move
    fn mov_e_c(&mut self) {

    }

    // Move
    fn mov_e_d(&mut self) {

    }

    // Move
    fn mov_e_h(&mut self) {

    }

    // Move
    fn mov_e_l(&mut self) {

    }

    // Move
    fn mov_e_m(&mut self) {

    }

    // Move
    fn mov_e_a(&mut self) {

    }

    // Move
    fn mov_h_b(&mut self) {

    }

    // Move
    fn mov_h_c(&mut self) {

    }

    // Move
    fn mov_h_d(&mut self) {

    }

    // Move
    fn mov_h_e(&mut self) {

    }

    // Move
    fn mov_h_l(&mut self) {

    }

    // Move
    fn mov_h_m(&mut self) {

    }

    // Move
    fn mov_h_a(&mut self) {

    }

    // Move
    fn mov_l_b(&mut self) {

    }

    // Move
    fn mov_l_c(&mut self) {

    }

    // Move
    fn mov_l_d(&mut self) {

    }

    // Move
    fn mov_l_e(&mut self) {

    }

    // Move
    fn mov_l_h(&mut self) {

    }

    // Move
    fn mov_l_m(&mut self) {

    }

    // Move
    fn mov_l_a(&mut self) {

    }

    // Move
    fn mov_m_b(&mut self) {

    }

    // Move
    fn mov_m_c(&mut self) {

    }

    // Move
    fn mov_m_d(&mut self) {

    }

    // Move
    fn mov_m_e(&mut self) {

    }

    // Move
    fn mov_m_h(&mut self) {

    }

    // Move
    fn mov_m_l(&mut self) {

    }

    // Move
    fn mov_m_a(&mut self) {

    }

    // Move
    fn mov_a_b(&mut self) {

    }

    // Move
    fn mov_a_c(&mut self) {

    }

    // Move
    fn mov_a_d(&mut self) {

    }

    // Move
    fn mov_a_e(&mut self) {

    }

    // Move
    fn mov_a_h(&mut self) {

    }

    // Move
    fn mov_a_l(&mut self) {

    }

    // Move
    fn mov_a_m(&mut self) {

    }

    // Store Accumulator
    fn stax_b(&mut self) {

    }

    // Store Accumulator
    fn stax_d(&mut self) {

    }

    // Load Accumulator
    fn ldax_b(&mut self) {

    }

    // Load Accumulator
    fn ldax_d(&mut self) {

    }

    // ADD Register or Memory To Accumulator
    fn add_b(&mut self) {

    }

    // ADD Register or Memory To Accumulator
    fn add_c(&mut self) {

    }

    // ADD Register or Memory To Accumulator
    fn add_d(&mut self) {

    }

    // ADD Register or Memory To Accumulator
    fn add_e(&mut self) {

    }

    // ADD Register or Memory To Accumulator
    fn add_h(&mut self) {

    }

    // ADD Register or Memory To Accumulator
    fn add_l(&mut self) {

    }

    // ADD Register or Memory To Accumulator
    fn add_m(&mut self) {

    }

    // ADD Register or Memory To Accumulator
    fn add_a(&mut self) {

    }

    // ADD Register or Memory to Accumulator with Carry
    fn adc_b(&mut self) {

    }

    // ADD Register or Memory to Accumulator with Carry
    fn adc_c(&mut self) {

    }

    // ADD Register or Memory to Accumulator with Carry
    fn adc_d(&mut self) {

    }

    // ADD Register or Memory to Accumulator with Carry
    fn adc_e(&mut self) {

    }

    // ADD Register or Memory to Accumulator with Carry
    fn adc_h(&mut self) {

    }

    // ADD Register or Memory to Accumulator with Carry
    fn adc_l(&mut self) {

    }

    // ADD Register or Memory to Accumulator with Carry
    fn adc_m(&mut self) {

    }

    // ADD Register or Memory to Accumulator with Carry
    fn adc_a(&mut self) {

    }

    // Subtract Register or Memory From Accumulator
    fn sub_b(&mut self) {

    }

    // Subtract Register or Memory From Accumulator
    fn sub_c(&mut self) {

    }

    // Subtract Register or Memory From Accumulator
    fn sub_d(&mut self) {

    }

    // Subtract Register or Memory From Accumulator
    fn sub_e(&mut self) {

    }

    // Subtract Register or Memory From Accumulator
    fn sub_h(&mut self) {

    }

    // Subtract Register or Memory From Accumulator
    fn sub_l(&mut self) {

    }

    // Subtract Register or Memory From Accumulator
    fn sub_m(&mut self) {

    }

    // Subtract Register or Memory From Accumulator
    fn sub_a(&mut self) {

    }

    // Subtract Register or Memory From Accumulator With Borrow
    fn sbb_b(&mut self) {

    }

    // Subtract Register or Memory From Accumulator With Borrow
    fn sbb_c(&mut self) {

    }

    // Subtract Register or Memory From Accumulator With Borrow
    fn sbb_d(&mut self) {

    }

    // Subtract Register or Memory From Accumulator With Borrow
    fn sbb_e(&mut self) {

    }

    // Subtract Register or Memory From Accumulator With Borrow
    fn sbb_h(&mut self) {

    }

    // Subtract Register or Memory From Accumulator With Borrow
    fn sbb_l(&mut self) {

    }

    // Subtract Register or Memory From Accumulator With Borrow
    fn sbb_m(&mut self) {

    }

    // Subtract Register or Memory From Accumulator With Borrow
    fn sbb_a(&mut self) {

    }

    // Logical and Register or Memory With Accumulator
    fn ana_b(&mut self) {

    }

    // Logical and Register or Memory With Accumulator
    fn ana_c(&mut self) {

    }

    // Logical and Register or Memory With Accumulator
    fn ana_d(&mut self) {

    }

    // Logical and Register or Memory With Accumulator
    fn ana_e(&mut self) {

    }

    // Logical and Register or Memory With Accumulator
    fn ana_h(&mut self) {

    }

    // Logical and Register or Memory With Accumulator
    fn ana_l(&mut self) {

    }

    // Logical and Register or Memory With Accumulator
    fn ana_m(&mut self) {

    }

    // Logical and Register or Memory With Accumulator
    fn ana_a(&mut self) {

    }

    // Logical Exclusive-Or Register or Memory With Accumulator (Zero Accumulator)
    fn xra_b(&mut self) {

    }

    // Logical Exclusive-Or Register or Memory With Accumulator (Zero Accumulator)
    fn xra_c(&mut self) {

    }

    // Logical Exclusive-Or Register or Memory With Accumulator (Zero Accumulator)
    fn xra_d(&mut self) {

    }

    // Logical Exclusive-Or Register or Memory With Accumulator (Zero Accumulator)
    fn xra_e(&mut self) {

    }

    // Logical Exclusive-Or Register or Memory With Accumulator (Zero Accumulator)
    fn xra_h(&mut self) {

    }

    // Logical Exclusive-Or Register or Memory With Accumulator (Zero Accumulator)
    fn xra_l(&mut self) {

    }

    // Logical Exclusive-Or Register or Memory With Accumulator (Zero Accumulator)
    fn xra_m(&mut self) {

    }

    // Logical Exclusive-Or Register or Memory With Accumulator (Zero Accumulator)
    fn xra_a(&mut self) {

    }

    // Logical or Register or Memory With Accumulator
    fn ora_b(&mut self) {

    }

    // Logical or Register or Memory With Accumulator
    fn ora_c(&mut self) {

    }

    // Logical or Register or Memory With Accumulator
    fn ora_d(&mut self) {

    }

    // Logical or Register or Memory With Accumulator
    fn ora_e(&mut self) {

    }

    // Logical or Register or Memory With Accumulator
    fn ora_h(&mut self) {

    }

    // Logical or Register or Memory With Accumulator
    fn ora_l(&mut self) {

    }

    // Logical or Register or Memory With Accumulator
    fn ora_m(&mut self) {

    }

    // Logical or Register or Memory With Accumulator
    fn ora_a(&mut self) {

    }

    // Compare Register or Memory With Accumulator
    fn cmp_b(&mut self) {

    }

    // Compare Register or Memory With Accumulator
    fn cmp_c(&mut self) {

    }

    // Compare Register or Memory With Accumulator
    fn cmp_d(&mut self) {

    }

    // Compare Register or Memory With Accumulator
    fn cmp_e(&mut self) {

    }

    // Compare Register or Memory With Accumulator
    fn cmp_h(&mut self) {

    }

    // Compare Register or Memory With Accumulator
    fn cmp_l(&mut self) {

    }

    // Compare Register or Memory With Accumulator
    fn cmp_m(&mut self) {

    }

    // Compare Register or Memory With Accumulator
    fn cmp_a(&mut self) {

    }

    // Rotate Accumulator Left
    fn rlc(&mut self) {

    }

    // Rotate Accumulator Right
    fn rrc(&mut self) {

    }

    // Rotate Accumulator Left Through Carry
    fn ral(&mut self) {

    }

    // Rotate Accumulator Right Through Carry
    fn rar(&mut self) {

    }

    // Push Data Onto Stack
    fn push_b(&mut self) {

    }

    // Push Data Onto Stack
    fn push_d(&mut self) {

    }

    // Push Data Onto Stack
    fn push_h(&mut self) {

    }

    // Push Data Onto Stack
    fn push_psw(&mut self) {

    }

    // Pop Data Off Stack
    fn pop_b(&mut self) {

    }

    // Pop Data Off Stack
    fn pop_d(&mut self) {

    }

    // Pop Data Off Stack
    fn pop_h(&mut self) {

    }

    // Pop Data Off Stack
    fn pop_psw(&mut self) {

    }

    // Double Add
    fn dad_b(&mut self) {

    }

    // Double Add
    fn dad_d(&mut self) {

    }

    // Double Add
    fn dad_h(&mut self) {

    }

    // Double Add
    fn dad_sp(&mut self) {

    }

    // Increment Register Pair
    fn inx_b(&mut self) {

    }

    // Increment Register Pair
    fn inx_d(&mut self) {

    }

    // Increment Register Pair
    fn inx_h(&mut self) {

    }

    // Increment Register Pair
    fn inx_sp(&mut self) {

    }

    // Decrement Register Pair
    fn dcx_b(&mut self) {

    }

    // Decrement Register Pair
    fn dcx_d(&mut self) {

    }

    // Decrement Register Pair
    fn dcx_h(&mut self) {

    }

    // Decrement Register Pair
    fn dcx_sp(&mut self) {

    }

    // Exchange Registers
    fn xchg(&mut self) {

    }

    // Exchange Stack
    fn xthl(&mut self) {

    }

    // Load SP From H and L
    fn sphl(&mut self) {

    }

    // Move Immediate Data
    fn lxi_b(&mut self) {

    }

    // Move Immediate Data
    fn lxi_d(&mut self) {

    }

    // Move Immediate Data
    fn lxi_h(&mut self) {

    }

    // Move Immediate Data
    fn lxi_sp(&mut self) {

    }

    // Move Immediate Data
    fn mvi_b(&mut self) {

    }

    // Move Immediate Data
    fn mvi_c(&mut self) {

    }

    // Move Immediate Data
    fn mvi_d(&mut self) {

    }

    // Move Immediate Data
    fn mvi_e(&mut self) {

    }

    // Move Immediate Data
    fn mvi_h(&mut self) {

    }

    // Move Immediate Data
    fn mvi_l(&mut self) {

    }

    // Move Immediate Data
    fn mvi_m(&mut self) {

    }

    // Move Immediate Data
    fn mvi_a(&mut self) {

    }

    // Add Immediate to Accumulator
    fn adi(&mut self) {

    }

    // Add Immediate to Accumulator With Carry
    fn aci(&mut self) {

    }

    // Subtract Immediate from Accumulator
    fn sui(&mut self) {

    }

    // Subtract Immediate from Accumulator With Borrow
    fn sbi(&mut self) {

    }

    // And Immediate With Accumulator
    fn ani(&mut self) {

    }

    // Exclusive-Or Immediate With Accumulator
    fn xri(&mut self) {

    }

    // Or Immediate With Accumulator 
    fn ori(&mut self) {

    }

    // Compare Immediate With Accumulator
    fn cpi(&mut self) {

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
