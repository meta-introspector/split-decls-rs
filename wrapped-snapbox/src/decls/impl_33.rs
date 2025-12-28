macro_rules! deps {
    () => {
        Assert!();
        Action!();
        Redactions!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        impl Assert { pub fn selected_action (& self) -> Action { self . action } pub fn redactions (& self) -> & crate :: Redactions { & self . substitutions } }
    };
}

impl_33!();