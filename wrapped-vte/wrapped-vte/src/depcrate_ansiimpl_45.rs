// Generated macro for impl_45 (impl)
macro_rules! Depcrate_ansiimpl_45 {
() => {
// Module: crate::ansi
// Provides: {"impl_45"}
// Dependencies: {}
impl FromStr for Rgb { type Err = () ; fn from_str (s : & str) -> Result < Rgb , () > { let chars = if s . starts_with ("0x") && s . len () == 8 { & s [2 ..] } else if s . starts_with ('#') && s . len () == 7 { & s [1 ..] } else { return Err (()) ; } ; match u32 :: from_str_radix (chars , 16) { Ok (mut color) => { let b = (color & 0xFF) as u8 ; color >>= 8 ; let g = (color & 0xFF) as u8 ; color >>= 8 ; let r = color as u8 ; Ok (Rgb { r , g , b }) } , Err (_) => Err (()) , } } }
};
}
