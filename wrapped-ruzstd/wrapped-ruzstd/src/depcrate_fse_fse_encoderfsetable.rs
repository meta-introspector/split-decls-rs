// Generated macro for FSETable (struct)
macro_rules! Depcrate_fse_fse_encoderFSETable {
() => {
// Module: crate::fse::fse_encoder
// Provides: {"FSETable"}
// Dependencies: {}
# [derive (Debug , Clone)] pub struct FSETable { # [doc = " Indexed by symbol"] pub (super) states : [SymbolStates ; 256] , # [doc = " Sum of all states.states.len()"] pub (crate) table_size : usize , }
};
}
