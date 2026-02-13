use byteorder::BigEndian;
use byteorder::WriteBytesExt;
use std::error::Error;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Write;

use crate::enum_builder;

use super::xex::CODE_START_CUSTOM_ADDRESS;

enum_builder! {
    #[repr(u32)]
    pub enum Functions {
        ChSmBottlesSkipIntroTutorial = 0x8218aab8,
        AbilitySetAllLearned = 0x8209ad78,
        FileProgressFlagSet = 0x820e62f0,
        FileProgressFlagSetN = 0x820e6390,
        CustomFunction = 0x82440cf4,
    }
}

pub fn prologue<W: Write + Seek>(writer: &mut W) -> Result<(), Box<dyn Error>> {
    // mflr r12
    writer.write_u32::<BigEndian>(0x7d8802a6)?;
    // stw r12, -8(r1)
    writer.write_u32::<BigEndian>(0x9181fff8)?;
    // stwu r1, -60h(r1)
    writer.write_u32::<BigEndian>(0x9421ffa0)?;

    Ok(())
}

pub fn epilogue<W: Write + Seek>(writer: &mut W) -> Result<(), Box<dyn Error>> {
    // addi r1, r1, 60h
    writer.write_u32::<BigEndian>(0x38210060)?;
    // lwz r12, -8(r1)
    writer.write_u32::<BigEndian>(0x8181fff8)?;
    // mtlr r12
    writer.write_u32::<BigEndian>(0x7d8803a6)?;

    // blr
    writer.write_u32::<BigEndian>(0x4e800020)?;

    Ok(())
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

pub fn set_flag<W: Write + Seek>(
    writer: &mut W,
    flag: u32,
    custom_address_start: u32,
) -> Result<(), Box<dyn Error>> {
    // li r4, 1
    writer.write_u32::<BigEndian>(0x38800001)?;

    // li r3, <flag>
    writer.write_u32::<BigEndian>(0x38600000 + flag)?;

    let current_custom_offset = writer.seek(SeekFrom::Current(0))?;
    let offset = (current_custom_offset - CODE_START_CUSTOM_ADDRESS) as u32;

    // bl fileProgressFlag_set
    writer.write_u32::<BigEndian>(call(
        custom_address_start + offset,
        Functions::FileProgressFlagSet,
    ))?;

    Ok(())
}

pub fn set_flags<W: Write + Seek>(
    writer: &mut W,
    start_flag: u32,
    length: u32,
    custom_address_start: u32,
) -> Result<(), Box<dyn Error>> {
    // li r4, <length>
    writer.write_u32::<BigEndian>(0x38a00000 + length)?;

    // li r4, 1
    writer.write_u32::<BigEndian>(0x38800001)?;

    // li r3, <flag>
    writer.write_u32::<BigEndian>(0x38600000 + start_flag)?;

    let current_custom_offset = writer.seek(SeekFrom::Current(0))?;
    let offset = (current_custom_offset - CODE_START_CUSTOM_ADDRESS) as u32;

    // bl fileProgressFlag_setN
    writer.write_u32::<BigEndian>(call(
        custom_address_start + offset,
        Functions::FileProgressFlagSetN,
    ))?;

    Ok(())
}
