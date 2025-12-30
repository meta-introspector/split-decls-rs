// Generated macro for interpret_cs_action (function)
macro_rules! Depcrate_sys_personality_dwarf_ehinterpret_cs_action {
() => {
// Module: crate::sys::personality::dwarf::eh
// Provides: {"interpret_cs_action"}
// Dependencies: {}
unsafe fn interpret_cs_action (action_table : * const u8 , cs_action_entry : u64 , lpad : LPad ,) -> EHAction { if cs_action_entry == 0 { EHAction :: Cleanup (lpad) } else { let action_record = unsafe { action_table . offset (cs_action_entry as isize - 1) } ; let mut action_reader = DwarfReader :: new (action_record) ; let ttype_index = unsafe { action_reader . read_sleb128 () } ; if ttype_index == 0 { EHAction :: Cleanup (lpad) } else if ttype_index > 0 { EHAction :: Catch (lpad) } else { EHAction :: Filter (lpad) } } }
};
}
