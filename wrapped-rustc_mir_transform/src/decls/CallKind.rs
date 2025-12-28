macro_rules! CallKind {
    () => {
        # [derive (Copy , Clone , Debug , PartialEq)] enum CallKind < 'tcx > { # [doc = " Call the `FnPtr` that was passed as the receiver."] Indirect (Ty < 'tcx >) , # [doc = " Call a known `FnDef`."] Direct (DefId) , }
    };
}

CallKind!()