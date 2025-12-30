// Generated macro for Container (struct)
macro_rules! DepcrateContainer {
() => {
// Module: crate
// Provides: {"Container"}
// Dependencies: {}
# [doc = " A container which is used for TLS."] pub struct Container < 'tcx , B : Bridge > { pub tables : RefCell < Tables < 'tcx , B > > , pub cx : RefCell < CompilerCtxt < 'tcx , B > > , }
};
}
