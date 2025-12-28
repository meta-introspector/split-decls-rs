macro_rules! FeatureConstraints {
    () => {
        # [derive (Copy , Clone , Debug)] pub struct FeatureConstraints { # [doc = " Features that must be enabled."] pub required : & 'static [& 'static str] , # [doc = " Features that must be disabled."] pub incompatible : & 'static [& 'static str] , }
    };
}

FeatureConstraints!()