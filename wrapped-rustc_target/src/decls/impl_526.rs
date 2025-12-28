macro_rules! deps {
    () => {
        TargetWarnings!();
    };
}

macro_rules! impl_526 {
    () => {
        deps!();
        impl TargetWarnings { pub fn empty () -> Self { Self { unused_fields : Vec :: new () } } pub fn warning_messages (& self) -> Vec < String > { let mut warnings = vec ! [] ; if ! self . unused_fields . is_empty () { warnings . push (format ! ("target json file contains unused fields: {}" , self . unused_fields . join (", "))) ; } warnings } }
    };
}

impl_526!();