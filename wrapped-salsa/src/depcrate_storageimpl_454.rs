// Generated macro for impl_454 (impl)
macro_rules! Depcrate_storageimpl_454 {
() => {
// Module: crate::storage
// Provides: {"impl_454"}
// Dependencies: {}
impl < Db : Database > StorageHandle < Db > { pub fn new (event_callback : Option < Box < dyn Fn (crate :: Event) + Send + Sync + 'static > >) -> Self { Self :: with_jars (event_callback , Vec :: new ()) } fn with_jars (event_callback : Option < Box < dyn Fn (crate :: Event) + Send + Sync + 'static > > , jars : Vec < ErasedJar > ,) -> Self { Self { zalsa_impl : Arc :: new (Zalsa :: new :: < Db > (event_callback , jars)) , coordinate : CoordinateDrop (Arc :: new (Coordinate { coordinate_lock : Mutex :: default () , cvar : Default :: default () , })) , phantom : PhantomData , } } pub fn into_storage (self) -> Storage < Db > { Storage { handle : self , zalsa_local : ZalsaLocal :: new () , } } }
};
}
