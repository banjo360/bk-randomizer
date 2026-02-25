use super::xex::CODE_START_CUSTOM_ADDRESS;
use crate::enum_builder;
use crate::enums::file_progress::FileProgress;
use byteorder::BigEndian;
use byteorder::WriteBytesExt;
use ppc::Block;
use ppc::Instruction;
use ppc::Register;
use std::error::Error;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Write;

enum_builder! {
    #[repr(u32)]
    pub enum Functions {
        ChSmBottlesSkipIntroTutorial = 0x8218aab8,
        AbilitySetAllLearned = 0x8209ad78,
        FileProgressFlagSet = 0x820e62f0,
        FileProgressFlagSetN = 0x820e6390,
        KeyPressed = 0x8209a818,
        CustomFunction = 0x82440cf4,
    }
}

pub fn prologue(block: &mut Block) {
    block.add(Instruction::Mflr(Register::R12));
    block.add(Instruction::Stw(Register::R12, Register::R1, -0x08));
    block.add(Instruction::Stw(Register::R1, Register::R1, -0x60));
}

pub fn epilogue(block: &mut Block) {
    block.add(Instruction::Addi(Register::R1, Register::R1, 0x60));
    block.add(Instruction::Lwz(Register::R12, Register::R1, -0x8));
    block.add(Instruction::Mtlr(Register::R12));
    block.add(Instruction::Blr);
}

pub fn call(address: u32, target: Functions) -> u32 {
    let target: u32 = target.into();

    jump(address, target) + 1
}

pub fn jump(address: u32, target: u32) -> u32 {
    let diff = target as i64 - address as i64;
    let inst = ((diff as u32) & 0x03FFFFFC) + 0x48000000;

    inst
}

pub fn nop<W: Write>(writer: &mut W) -> Result<(), Box<dyn Error>> {
    // ori r0, r0, 0
    // i.e. "noop"
    let inst = Instruction::Ori(Register::R0, Register::R0, 0);
    inst.write(writer, 0)?;

    Ok(())
}

pub fn set_flag(block: &mut Block, flag: FileProgress) {
    let flag: u32 = flag.into();
    let func: u32 = Functions::FileProgressFlagSet.into();

    block.add(Instruction::Li(Register::R4, 1));
    block.add(Instruction::Li(Register::R3, flag as u16));
    block.add(Instruction::Bl(func));
}

pub fn set_flags(block: &mut Block, start_flag: FileProgress, length: u32) {
    let start_flag: u32 = start_flag.into();

    let func: u32 = Functions::FileProgressFlagSetN.into();

    block.add(Instruction::Li(Register::R5, length as u16));
    let bits = (u16::MAX >> (16 - length));
    block.add(Instruction::Li(Register::R4, bits));
    block.add(Instruction::Li(Register::R3, start_flag as u16));
    block.add(Instruction::Bl(func));
}
