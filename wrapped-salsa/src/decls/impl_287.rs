macro_rules! deps {
    () => {
        Coordinate!();
        Zalsa!();
        Event!();
        Storage!();
        ZalsaLocal!();
        ErasedJar!();
        CoordinateDrop!();
        StorageHandle!();
        Database!();
    };
}

macro_rules! impl_287 {
    () => {
        deps!();
        impl < Db : Database > StorageHandle < Db > { pub fn new (event_callback : Option < Box < dyn Fn (crate :: Event) + Send + Sync + 'static > >) -> Self { Self :: with_jars (event_callback , Vec :: new ()) } fn with_jars (event_callback : Option < Box < dyn Fn (crate :: Event) + Send + Sync + 'static > > , jars : Vec < ErasedJar > ,) -> Self { Self { zalsa_impl : Arc :: new (Zalsa :: new :: < Db > (event_callback , jars)) , coordinate : CoordinateDrop (Arc :: new (Coordinate { coordinate_lock : Mutex :: default () , cvar : Default :: default () , })) , phantom : PhantomData , } } pub fn into_storage (self) -> Storage < Db > { Storage { handle : self , zalsa_local : ZalsaLocal :: new () , } } }
    };
}

impl_287!()