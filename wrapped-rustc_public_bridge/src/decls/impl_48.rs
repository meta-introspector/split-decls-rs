macro_rules! deps {
    () => {
        CompilerCtxt!();
        Bridge!();
    };
}

macro_rules! impl_48 {
    () => {
        deps!();
        impl < 'tcx , B : Bridge > HasTypingEnv < 'tcx > for CompilerCtxt < 'tcx , B > { fn typing_env (& self) -> ty :: TypingEnv < 'tcx > { ty :: TypingEnv :: fully_monomorphized () } }
    };
}

impl_48!()