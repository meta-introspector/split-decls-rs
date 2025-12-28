macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! impl_159 {
    () => {
        deps!();
        # [cfg (all (tracing_unstable , feature = "valuable"))] impl crate :: sealed :: Sealed for valuable :: Value < '_ > { }
    };
}

impl_159!()