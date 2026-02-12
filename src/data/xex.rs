pub const LAIR_WARPS_TARGET: u64 = calculate_data_file_offset(0x82455d90);
pub const OPENED_LEVELS_FLAGS: u64 = calculate_data_file_offset(0x8246d0b4);
pub const MOLEHILLS_MOVES_DATA: u64 = calculate_data_file_offset(0x82466d48);
pub const NOTE_DOORS_COSTS_ADDRESS: u64 = calculate_data_file_offset(0x8246d0dc);
pub const CODE_START_CUSTOM_ADDRESS: u64 = 0x442cf4; // 82440cf4

const fn calculate_data_file_offset(address: u64) -> u64 {
    if address < 0x82450000 || address > 0x825085af {
        unreachable!();
    } else {
        address - 0x82006000
    }
}
