// Generated macro for Motherboard (struct)
macro_rules! Depcrate_common_systemMotherboard {
() => {
// Module: crate::common::system
// Provides: {"Motherboard"}
// Dependencies: {}
# [doc = " This type allows to retrieve motherboard-related information."] # [doc = ""] # [doc = " ```"] # [doc = " use sysinfo::Motherboard;"] # [doc = ""] # [doc = " if let Some(m) = Motherboard::new() {"] # [doc = "     println!(\"{m:?}\");"] # [doc = " }"] # [doc = " ```"] pub struct Motherboard { pub (crate) inner : MotherboardInner , }
};
}
