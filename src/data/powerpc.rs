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
        CustomFunction = 0x82440cf4,
    }
}

pub fn call(address: u32, target: Functions) -> u32 {
    let target: u32 = target.into();
    let diff = target as i64 - address as i64;
    let inst = ((diff as u32) & 0x03FFFFFC) + 0x48000001;

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
    ));

    Ok(())
}

pub fn blr() -> u32 {
    0x4e800020
}
