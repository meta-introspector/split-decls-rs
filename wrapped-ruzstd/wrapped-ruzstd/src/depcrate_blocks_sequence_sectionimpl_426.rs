// Generated macro for impl_426 (impl)
macro_rules! Depcrate_blocks_sequence_sectionimpl_426 {
() => {
// Module: crate::blocks::sequence_section
// Provides: {"impl_426"}
// Dependencies: {}
impl CompressionModes { # [doc = " Deserialize a two bit mode value into a [ModeType]"] pub fn decode_mode (m : u8) -> ModeType { match m { 0 => ModeType :: Predefined , 1 => ModeType :: RLE , 2 => ModeType :: FSECompressed , 3 => ModeType :: Repeat , _ => panic ! ("This can never happen") , } } # [doc = " Read the compression mode of the literal lengths field."] pub fn ll_mode (self) -> ModeType { Self :: decode_mode (self . 0 >> 6) } # [doc = " Read the compression mode of the offset value field."] pub fn of_mode (self) -> ModeType { Self :: decode_mode ((self . 0 >> 4) & 0x3) } # [doc = " Read the compression mode of the match lengths field."] pub fn ml_mode (self) -> ModeType { Self :: decode_mode ((self . 0 >> 2) & 0x3) } }
};
}
