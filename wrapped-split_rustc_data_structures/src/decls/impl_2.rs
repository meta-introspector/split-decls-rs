macro_rules! deps {
    () => {
        Aligned!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        unsafe impl < T > Aligned for T { const ALIGN : Alignment = Alignment :: of :: < Self > () ; }
    };
}

impl_2!();