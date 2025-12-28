macro_rules! deps {
    () => {
        DetectionState!();
        Builder!();
    };
}

macro_rules! impl_99 {
    () => {
        deps!();
        impl Default for DetectionState { fn default () -> Self { Builder :: default () . into_detection_state () . expect ("enabled by default") } }
    };
}

impl_99!()