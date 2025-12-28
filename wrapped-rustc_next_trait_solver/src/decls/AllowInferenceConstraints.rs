macro_rules! AllowInferenceConstraints {
    () => {
        pub (super) enum AllowInferenceConstraints { Yes , No , }
    };
}

AllowInferenceConstraints!()