macro_rules! deps {
    () => {
        LintId!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl PartialEq for LintId { fn eq (& self , other : & LintId) -> bool { std :: ptr :: eq (self . lint , other . lint) } }
    };
}

impl_18!()