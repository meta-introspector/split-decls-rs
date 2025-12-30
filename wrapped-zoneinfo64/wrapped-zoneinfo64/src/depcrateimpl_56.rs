// Generated macro for impl_56 (impl)
macro_rules! Depcrateimpl_56 {
() => {
// Module: crate
// Provides: {"impl_56"}
// Dependencies: {}
impl PartialEq for Zone < '_ > { fn eq (& self , other : & Self) -> bool { self . name () == other . name () && self . region () == other . region () && self . simple () . trans == other . simple () . trans && self . simple () . trans_post32 == other . simple () . trans_post32 && self . simple () . trans_pre32 == other . simple () . trans_pre32 && self . simple () . type_map == other . simple () . type_map && self . simple () . type_offsets == other . simple () . type_offsets && self . simple () . links == other . simple () . links && self . simple () . final_rule (& self . info . rules) == other . simple () . final_rule (& other . info . rules) } }
};
}
