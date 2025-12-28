macro_rules! deps {
    () => {
        Storage!();
        Database!();
    };
}

macro_rules! impl_291 {
    () => {
        deps!();
        impl < Db : Database > Default for Storage < Db > { fn default () -> Self { Self :: new (None) } }
    };
}

impl_291!();