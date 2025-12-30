// Generated macro for Ctxt (struct)
macro_rules! Depcrate_internals_ctxtCtxt {
() => {
// Module: crate::internals::ctxt
// Provides: {"Ctxt"}
// Dependencies: {}
# [doc = " A type to collect errors together and format them."] # [doc = ""] # [doc = " Dropping this object will cause a panic. It must be consumed using `check`."] # [doc = ""] # [doc = " References can be shared since this type uses run-time exclusive mut checking."] # [derive (Default)] pub struct Ctxt { errors : RefCell < Option < Vec < syn :: Error > > > , }
};
}
