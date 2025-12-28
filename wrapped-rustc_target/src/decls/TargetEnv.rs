macro_rules! TargetEnv {
    () => {
        # [derive (Copy , Clone , PartialEq)] pub (crate) enum TargetEnv { Normal , Simulator , MacCatalyst , }
    };
}

TargetEnv!()