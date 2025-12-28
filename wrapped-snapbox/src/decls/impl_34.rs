macro_rules! deps {
    () => {
        Assert!();
        Redactions!();
        Palette!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        impl Default for Assert { fn default () -> Self { Self { action : Default :: default () , action_var : Default :: default () , normalize_paths : true , substitutions : Default :: default () , palette : crate :: report :: Palette :: color () , } . redact_with (crate :: Redactions :: with_exe ()) } }
    };
}

impl_34!();