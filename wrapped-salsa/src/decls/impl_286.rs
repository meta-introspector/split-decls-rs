macro_rules! deps {
    () => {
        Database!();
        StorageHandle!();
    };
}

macro_rules! impl_286 {
    () => {
        deps!();
        impl < Db : Database > Default for StorageHandle < Db > { fn default () -> Self { Self :: new (None) } }
    };
}

impl_286!()