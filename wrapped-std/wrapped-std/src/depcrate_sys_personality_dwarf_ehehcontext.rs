// Generated macro for EHContext (struct)
macro_rules! Depcrate_sys_personality_dwarf_ehEHContext {
() => {
// Module: crate::sys::personality::dwarf::eh
// Provides: {"EHContext"}
// Dependencies: {}
# [derive (Copy , Clone)] pub struct EHContext < 'a > { pub ip : * const u8 , pub func_start : * const u8 , pub get_text_start : & 'a dyn Fn () -> * const u8 , pub get_data_start : & 'a dyn Fn () -> * const u8 , }
};
}
