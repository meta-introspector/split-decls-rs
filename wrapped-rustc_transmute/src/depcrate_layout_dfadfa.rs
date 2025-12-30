// Generated macro for Dfa (struct)
macro_rules! Depcrate_layout_dfaDfa {
() => {
// Module: crate::layout::dfa
// Provides: {"Dfa"}
// Dependencies: {}
# [derive (PartialEq)] # [cfg_attr (test , derive (Clone))] pub (crate) struct Dfa < R , T > where R : Region , T : Type , { pub (crate) transitions : Map < State , Transitions < R , T > > , pub (crate) start : State , pub (crate) accept : State , }
};
}
