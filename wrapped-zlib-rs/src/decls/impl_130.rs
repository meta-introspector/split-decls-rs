macro_rules! deps {
    () => {
        DeflateConfig!();
    };
}

macro_rules! impl_130 {
    () => {
        deps!();
        impl DeflateConfig { pub fn new (level : i32) -> Self { Self { level , .. Self :: default () } } }
    };
}

impl_130!()