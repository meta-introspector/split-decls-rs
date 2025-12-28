macro_rules! deps {
    () => {
        LintId!();
    };
}

macro_rules! impl_154 {
    () => {
        deps!();
        impl PartialEq for LintId { fn eq (& self , other : & LintId) -> bool { std :: ptr :: eq (self . lint , other . lint) } }
    };
}

impl_154!()