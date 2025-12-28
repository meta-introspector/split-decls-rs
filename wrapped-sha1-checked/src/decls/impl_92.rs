macro_rules! deps {
    () => {
        DetectionState!();
    };
}

macro_rules! impl_92 {
    () => {
        deps!();
        impl Drop for DetectionState { # [inline] fn drop (& mut self) { # [cfg (feature = "zeroize")] { self . ihv1 . zeroize () ; self . ihv2 . zeroize () ; self . m1 . zeroize () ; self . m2 . zeroize () ; self . state_58 . zeroize () ; self . state_65 . zeroize () ; } } }
    };
}

impl_92!();