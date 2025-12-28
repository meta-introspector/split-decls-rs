macro_rules! TargetKind {
    () => {
        # [doc = " For the [`Target::check_consistency`] function, determines whether the given target is a builtin or a JSON"] # [doc = " target."] # [derive (Copy , Clone , Debug , PartialEq)] enum TargetKind { Json , Builtin , }
    };
}

TargetKind!()