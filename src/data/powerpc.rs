pub fn call(address: u32, target: u32) -> u32 {
    let diff = target as i64 - address as i64;
    let inst = ((diff as u32) & 0x03FFFFFC) + 0x48000001;

    inst
}

pub fn blr() -> u32 {
    0x4e800020
}
