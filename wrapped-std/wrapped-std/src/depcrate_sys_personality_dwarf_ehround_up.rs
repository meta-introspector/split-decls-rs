// Generated macro for round_up (function)
macro_rules! Depcrate_sys_personality_dwarf_ehround_up {
() => {
// Module: crate::sys::personality::dwarf::eh
// Provides: {"round_up"}
// Dependencies: {}
# [inline] fn round_up (unrounded : usize , align : usize) -> Result < usize , () > { if align . is_power_of_two () { Ok ((unrounded + align - 1) & ! (align - 1)) } else { Err (()) } }
};
}
