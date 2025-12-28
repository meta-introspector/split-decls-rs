macro_rules! deps {
    () => {
        Scan!();
    };
}

macro_rules! impl_285 {
    () => {
        deps!();
        # [cfg (feature = "full")] impl Clone for Scan { fn clone (& self) -> Self { * self } }
    };
}

impl_285!()