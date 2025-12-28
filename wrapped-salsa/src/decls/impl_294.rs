macro_rules! deps {
    () => {
        StorageBuilder!();
    };
}

macro_rules! impl_294 {
    () => {
        deps!();
        impl < Db > Default for StorageBuilder < Db > { fn default () -> Self { Self { jars : Vec :: new () , event_callback : None , _db : PhantomData , } } }
    };
}

impl_294!();