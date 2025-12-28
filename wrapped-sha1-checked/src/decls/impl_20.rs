macro_rules! deps {
    () => {
        DetectionState!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        # [cfg (feature = "zeroize")] impl ZeroizeOnDrop for DetectionState { }
    };
}

impl_20!()