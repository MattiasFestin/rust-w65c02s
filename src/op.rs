//! Opcode constants, for embedding vaguely readable machine code in Rust code.
//!
//! These constants either take the form `<op>` or `<op>_<mode>`. `<op>` is an
//! instruction mnemonic. `<mode>` is the addressing mode.
//!
//! Example: `ADC_IMM` = `ADC` instruction in Immediate addressing mode.

pub const BRK: u8 = 0x00;
pub const ORA_ZPXI: u8 = 0x01;
pub const TSB_ZP: u8 = 0x04;
pub const ORA_ZP: u8 = 0x05;
pub const ASL_ZP: u8 = 0x06;
pub const RMB0_ZP: u8 = 0x07;
pub const PHP: u8 = 0x08;
pub const ORA_IMM: u8 = 0x09;
pub const ASL_A: u8 = 0x0A;
pub const TSB_ABS: u8 = 0x0C;
pub const ORA_ABS: u8 = 0x0D;
pub const ASL_ABS: u8 = 0x0E;
pub const BBR0: u8 = 0x0F;
pub const BPL: u8 = 0x10;
pub const ORA_ZPIY: u8 = 0x11;
pub const ORA_ZPI: u8 = 0x12;
pub const TRB_ZP: u8 = 0x14;
pub const ORA_ZPX: u8 = 0x15;
pub const ASL_ZPX: u8 = 0x16;
pub const RMB1_ZP: u8 = 0x17;
pub const CLC: u8 = 0x18;
pub const ORA_ABSY: u8 = 0x19;
pub const INC_A: u8 = 0x1A;
pub const TRB_ABS: u8 = 0x1C;
pub const ORA_ABSX: u8 = 0x1D;
pub const ASL_ABSX: u8 = 0x1E;
pub const BBR1: u8 = 0x1F;
pub const JSR: u8 = 0x20;
pub const AND_ZPXI: u8 = 0x21;
pub const BIT_ZP: u8 = 0x24;
pub const AND_ZP: u8 = 0x25;
pub const ROL_ZP: u8 = 0x26;
pub const RMB2_ZP: u8 = 0x27;
pub const PLP: u8 = 0x28;
pub const AND_IMM: u8 = 0x29;
pub const ROL_A: u8 = 0x2A;
pub const BIT_ABS: u8 = 0x2C;
pub const AND_ABS: u8 = 0x2D;
pub const ROL_ABS: u8 = 0x2E;
pub const BBR2: u8 = 0x2F;
pub const BMI: u8 = 0x30;
pub const AND_ZPIY: u8 = 0x31;
pub const AND_ZPI: u8 = 0x32;
pub const BIT_ZPX: u8 = 0x34;
pub const AND_ZPX: u8 = 0x35;
pub const ROL_ZPX: u8 = 0x36;
pub const RMB3_ZP: u8 = 0x37;
pub const SEC: u8 = 0x38;
pub const AND_ABSY: u8 = 0x39;
pub const DEC_A: u8 = 0x3A;
pub const BIT_ABSX: u8 = 0x3C;
pub const AND_ABSX: u8 = 0x3D;
pub const ROL_ABSX: u8 = 0x3E;
pub const BBR3: u8 = 0x3F;
pub const RTI: u8 = 0x40;
pub const EOR_ZPXI: u8 = 0x41;
pub const NOP_ZP: u8 = 0x44;
pub const EOR_ZP: u8 = 0x45;
pub const LSR_ZP: u8 = 0x46;
pub const RMB4_ZP: u8 = 0x47;
pub const PHA: u8 = 0x48;
pub const EOR_IMM: u8 = 0x49;
pub const LSR_A: u8 = 0x4A;
pub const JMP_ABS: u8 = 0x4C;
pub const EOR_ABS: u8 = 0x4D;
pub const LSR_ABS: u8 = 0x4E;
pub const BBR4: u8 = 0x4F;
pub const BVC: u8 = 0x50;
pub const EOR_ZPIY: u8 = 0x51;
pub const EOR_ZPI: u8 = 0x52;
pub const EOR_ZPX: u8 = 0x55;
pub const LSR_ZPX: u8 = 0x56;
pub const RMB5_ZP: u8 = 0x57;
pub const CLI: u8 = 0x58;
pub const EOR_ABSY: u8 = 0x59;
pub const PHY: u8 = 0x5A;
pub const EOR_ABSX: u8 = 0x5D;
pub const LSR_ABSX: u8 = 0x5E;
pub const BBR5: u8 = 0x5F;
pub const RTS: u8 = 0x60;
pub const ADC_ZPXI: u8 = 0x61;
pub const STZ_ZP: u8 = 0x64;
pub const ADC_ZP: u8 = 0x65;
pub const ROR_ZP: u8 = 0x66;
pub const RMB6_ZP: u8 = 0x67;
pub const PLA: u8 = 0x68;
pub const ADC_IMM: u8 = 0x69;
pub const ROR_A: u8 = 0x6A;
pub const JMP_ABSI: u8 = 0x6C;
pub const ADC_ABS: u8 = 0x6D;
pub const ROR_ABS: u8 = 0x6E;
pub const BBR6: u8 = 0x6F;
pub const BVS: u8 = 0x70;
pub const ADC_ZPIY: u8 = 0x71;
pub const ADC_ZPI: u8 = 0x72;
pub const STZ_ZPX: u8 = 0x74;
pub const ADC_ZPX: u8 = 0x75;
pub const ROR_ZPX: u8 = 0x76;
pub const RMB7_ZP: u8 = 0x77;
pub const SEI: u8 = 0x78;
pub const ADC_ABSY: u8 = 0x79;
pub const PLY: u8 = 0x7A;
pub const JMP_ABSXI: u8 = 0x7C;
pub const ADC_ABSX: u8 = 0x7D;
pub const ROR_ABSX: u8 = 0x7E;
pub const BBR7: u8 = 0x7F;
pub const BRA: u8 = 0x80;
pub const STA_ZPXI: u8 = 0x81;
pub const STY_ZP: u8 = 0x84;
pub const STA_ZP: u8 = 0x85;
pub const STX_ZP: u8 = 0x86;
pub const SMB0_ZP: u8 = 0x87;
pub const DEC_Y: u8 = 0x88;
pub const BIT_IMM: u8 = 0x89;
pub const TXA: u8 = 0x8A;
pub const STY_ABS: u8 = 0x8C;
pub const STA_ABS: u8 = 0x8D;
pub const STX_ABS: u8 = 0x8E;
pub const BBS0: u8 = 0x8F;
pub const BCC: u8 = 0x90;
pub const STA_ZPIY: u8 = 0x91;
pub const STA_ZPI: u8 = 0x92;
pub const STY_ZPX: u8 = 0x94;
pub const STA_ZPX: u8 = 0x95;
pub const STX_ZPY: u8 = 0x96;
pub const SMB1_ZP: u8 = 0x97;
pub const TYA: u8 = 0x98;
pub const STA_ABSY: u8 = 0x99;
pub const TXS: u8 = 0x9A;
pub const STZ_ABS: u8 = 0x9C;
pub const STA_ABSX: u8 = 0x9D;
pub const STZ_ABSX: u8 = 0x9E;
pub const BBS1: u8 = 0x9F;
pub const LDY_IMM: u8 = 0xA0;
pub const LDA_ZPXI: u8 = 0xA1;
pub const LDX_IMM: u8 = 0xA2;
pub const LDY_ZP: u8 = 0xA4;
pub const LDA_ZP: u8 = 0xA5;
pub const LDX_ZP: u8 = 0xA6;
pub const SMB2_ZP: u8 = 0xA7;
pub const TAY: u8 = 0xA8;
pub const LDA_IMM: u8 = 0xA9;
pub const TAX: u8 = 0xAA;
pub const LDY_ABS: u8 = 0xAC;
pub const LDA_ABS: u8 = 0xAD;
pub const LDX_ABS: u8 = 0xAE;
pub const BBS2: u8 = 0xAF;
pub const BCS: u8 = 0xB0;
pub const LDA_ZPIY: u8 = 0xB1;
pub const LDA_ZPI: u8 = 0xB2;
pub const LDY_ZPX: u8 = 0xB4;
pub const LDA_ZPX: u8 = 0xB5;
pub const LDX_ZPY: u8 = 0xB6;
pub const SMB3_ZP: u8 = 0xB7;
pub const CLV: u8 = 0xB8;
pub const LDA_ABSY: u8 = 0xB9;
pub const TSX: u8 = 0xBA;
pub const LDY_ABSX: u8 = 0xBC;
pub const LDA_ABSX: u8 = 0xBD;
pub const LDX_ABSY: u8 = 0xBE;
pub const BBS3: u8 = 0xBF;
pub const CPY_IMM: u8 = 0xC0;
pub const CMP_ZPXI: u8 = 0xC1;
pub const CPY_ZP: u8 = 0xC4;
pub const CMP_ZP: u8 = 0xC5;
pub const DEC_ZP: u8 = 0xC6;
pub const SMB4_ZP: u8 = 0xC7;
pub const INC_Y: u8 = 0xC8;
pub const CMP_IMM: u8 = 0xC9;
pub const DEC_X: u8 = 0xCA;
pub const WAI: u8 = 0xCB;
pub const CPY_ABS: u8 = 0xCC;
pub const CMP_ABS: u8 = 0xCD;
pub const DEC_ABS: u8 = 0xCE;
pub const BBS4: u8 = 0xCF;
pub const BNE: u8 = 0xD0;
pub const CMP_ZPIY: u8 = 0xD1;
pub const CMP_ZPI: u8 = 0xD2;
pub const CMP_ZPX: u8 = 0xD5;
pub const DEC_ZPX: u8 = 0xD6;
pub const SMB5_ZP: u8 = 0xD7;
pub const CLD: u8 = 0xD8;
pub const CMP_ABSY: u8 = 0xD9;
pub const PHX: u8 = 0xDA;
pub const STP: u8 = 0xDB;
pub const CMP_ABSX: u8 = 0xDD;
pub const DEC_ABSX: u8 = 0xDE;
pub const BBS5: u8 = 0xDF;
pub const CPX_IMM: u8 = 0xE0;
pub const SBC_ZPXI: u8 = 0xE1;
pub const NOP_IMM: u8 = 0xE2;
pub const CPX_ZP: u8 = 0xE4;
pub const SBC_ZP: u8 = 0xE5;
pub const INC_ZP: u8 = 0xE6;
pub const SMB6_ZP: u8 = 0xE7;
pub const INC_X: u8 = 0xE8;
pub const SBC_IMM: u8 = 0xE9;
pub const NOP: u8 = 0xEA;
pub const CPX_ABS: u8 = 0xEC;
pub const SBC_ABS: u8 = 0xED;
pub const INC_ABS: u8 = 0xEE;
pub const BBS6: u8 = 0xEF;
pub const BEQ: u8 = 0xF0;
pub const SBC_ZPIY: u8 = 0xF1;
pub const SBC_ZPI: u8 = 0xF2;
pub const NOP_ZPX: u8 = 0xF4;
pub const SBC_ZPX: u8 = 0xF5;
pub const INC_ZPX: u8 = 0xF6;
pub const SMB7_ZP: u8 = 0xF7;
pub const SED: u8 = 0xF8;
pub const SBC_ABSY: u8 = 0xF9;
pub const PLX: u8 = 0xFA;
pub const NOP_ABS: u8 = 0xFC;
pub const SBC_ABSX: u8 = 0xFD;
pub const INC_ABSX: u8 = 0xFE;
pub const BBS7: u8 = 0xFF;