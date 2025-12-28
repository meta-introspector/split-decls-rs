macro_rules! deps {
    () => {
        UnwrapLayoutCx!();
    };
}

macro_rules! impl_273 {
    () => {
        deps!();
        impl < 'tcx > HasTypingEnv < 'tcx > for UnwrapLayoutCx < 'tcx > { fn typing_env (& self) -> ty :: TypingEnv < 'tcx > { self . typing_env } }
    };
}

impl_273!()