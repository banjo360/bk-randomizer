pub const LAIR_WARPS_TARGET: u64 = calculate_file_offset(0x82455D90);
pub const OPENED_LEVELS_FLAGS: u64 = calculate_file_offset(0x8246D0B4);
pub const MOLEHILLS_MOVES_DATA: u64 = calculate_file_offset(0x82466d48);

const fn calculate_file_offset(address: u64) -> u64 {
    if address < 0x82450000 || address > 0x825085af {
        unreachable!();
    } else {
        address - 0x82006000
    }
}
