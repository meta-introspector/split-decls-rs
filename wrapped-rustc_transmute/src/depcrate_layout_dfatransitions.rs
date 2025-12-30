// Generated macro for Transitions (struct)
macro_rules! Depcrate_layout_dfaTransitions {
() => {
// Module: crate::layout::dfa
// Provides: {"Transitions"}
// Dependencies: {}
# [derive (PartialEq , Clone , Debug)] pub (crate) struct Transitions < R , T > where R : Region , T : Type , { byte_transitions : EdgeSet < State > , ref_transitions : Map < Reference < R , T > , State > , }
};
}
