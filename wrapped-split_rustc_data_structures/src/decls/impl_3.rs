macro_rules! deps {
    () => {
        Aligned!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        unsafe impl < T > Aligned for [T] { const ALIGN : Alignment = Alignment :: of :: < T > () ; }
    };
}

impl_3!();