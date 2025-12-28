macro_rules! deps {
    () => {
        DetectionState!();
    };
}

macro_rules! impl_93 {
    () => {
        deps!();
        # [cfg (feature = "zeroize")] impl ZeroizeOnDrop for DetectionState { }
    };
}

impl_93!()