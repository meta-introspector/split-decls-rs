macro_rules! macro_12 {
    () => {
        feature ! { #! [all (feature = "registry" , feature = "std")] pub use registry :: Registry ; # [doc = " Returns a default [`Registry`]."] pub fn registry () -> Registry { Registry :: default () } }
    };
}

macro_12!()