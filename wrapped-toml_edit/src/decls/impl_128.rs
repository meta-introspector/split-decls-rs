macro_rules! deps {
    () => {
        Key!();
    };
}

macro_rules! impl_128 {
    () => {
        deps!();
        impl Clone for Key { # [inline (never)] fn clone (& self) -> Self { Self { key : self . key . clone () , repr : self . repr . clone () , leaf_decor : self . leaf_decor . clone () , dotted_decor : self . dotted_decor . clone () , } } }
    };
}

impl_128!();