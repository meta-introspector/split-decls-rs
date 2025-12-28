macro_rules! UnsupportedReprError {
    () => {
        # [doc = " The representation hint is not supported for the decorated type."] # [cfg_attr (test , derive (Copy , Clone , Debug , Eq , PartialEq))] pub (crate) struct UnsupportedReprError ;
    };
}

UnsupportedReprError!()