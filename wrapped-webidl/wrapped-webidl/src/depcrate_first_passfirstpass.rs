// Generated macro for FirstPass (trait)
macro_rules! Depcrate_first_passFirstPass {
() => {
// Module: crate::first_pass
// Provides: {"FirstPass"}
// Dependencies: {}
# [doc = " Implemented on an AST node to populate the `FirstPassRecord` struct."] pub (crate) trait FirstPass < 'src , Ctx > { # [doc = " Populate `record` with any constructs in `self`."] fn first_pass (& 'src self , record : & mut FirstPassRecord < 'src > , ctx : Ctx) -> Result < () > ; }
};
}
