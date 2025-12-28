macro_rules! deps {
    () => {
        TargetEnv!();
    };
}

macro_rules! impl_340 {
    () => {
        deps!();
        impl TargetEnv { fn target_env (self) -> & 'static str { match self { Self :: Normal => "" , Self :: MacCatalyst => "macabi" , Self :: Simulator => "sim" , } } }
    };
}

impl_340!();