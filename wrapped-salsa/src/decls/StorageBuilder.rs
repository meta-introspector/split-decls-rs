macro_rules! deps {
    () => {
        Storage!();
        Event!();
        ErasedJar!();
    };
}

macro_rules! StorageBuilder {
    () => {
        deps!();
        # [doc = " A builder for a [`Storage`] instance."] # [doc = ""] # [doc = " This type can be created with the [`Storage::builder`] function."] pub struct StorageBuilder < Db > { jars : Vec < ErasedJar > , event_callback : Option < Box < dyn Fn (crate :: Event) + Send + Sync + 'static > > , _db : PhantomData < Db > , }
    };
}

StorageBuilder!();