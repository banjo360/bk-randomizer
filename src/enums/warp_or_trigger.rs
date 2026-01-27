use crate::enum_builder;

enum_builder! {
    #[repr(u16)]
    pub enum WarpOrTriggerId {
        TtcEnterLevel = 12,
        CcEnterLevel = 13,
        BgsEnterLevel = 14,
        GvEnterLevel = 15,
        MmmEnterLevel = 16,
        RbbEnterLevel = 17,
        FpEnterLevel = 115,
        MmEnterLevel = 159,
        CcwEnterLevel = 290,
    }
}
