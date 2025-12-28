macro_rules! deps {
    () => {
        CompilerCtxt!();
        TypingEnvHelpers!();
        Bridge!();
    };
}

macro_rules! impl_40 {
    () => {
        deps!();
        impl < 'tcx , B : Bridge > TypingEnvHelpers < 'tcx > for CompilerCtxt < 'tcx , B > { fn fully_monomorphized (& self) -> ty :: TypingEnv < 'tcx > { ty :: TypingEnv :: fully_monomorphized () } }
    };
}

impl_40!();