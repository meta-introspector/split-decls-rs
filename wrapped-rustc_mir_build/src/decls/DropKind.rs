macro_rules! DropKind {
    () => {
        # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub (crate) enum DropKind { Value , Storage , ForLint (BackwardIncompatibleDropReason) , }
    };
}

DropKind!();