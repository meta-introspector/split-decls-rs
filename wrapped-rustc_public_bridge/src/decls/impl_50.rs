macro_rules! deps {
    () => {
        CompilerCtxt!();
        Bridge!();
    };
}

macro_rules! impl_50 {
    () => {
        deps!();
        impl < 'tcx , B : Bridge > HasDataLayout for CompilerCtxt < 'tcx , B > { fn data_layout (& self) -> & rustc_abi :: TargetDataLayout { self . tcx . data_layout () } }
    };
}

impl_50!()