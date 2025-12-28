macro_rules! deps {
    () => {
        BranchPatUsefulness!();
        PatCx!();
    };
}

macro_rules! impl_89 {
    () => {
        deps!();
        impl < 'p , Cx : PatCx > Default for BranchPatUsefulness < 'p , Cx > { fn default () -> Self { Self { useful : Default :: default () , covered_by : Default :: default () } } }
    };
}

impl_89!();