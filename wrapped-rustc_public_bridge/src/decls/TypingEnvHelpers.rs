macro_rules! TypingEnvHelpers {
    () => {
        pub trait TypingEnvHelpers < 'tcx > { fn fully_monomorphized (& self) -> ty :: TypingEnv < 'tcx > ; }
    };
}

TypingEnvHelpers!();