// Generated macro for impl_462 (impl)
macro_rules! Depcrate_storageimpl_462 {
() => {
// Module: crate::storage
// Provides: {"impl_462"}
// Dependencies: {}
impl < Db : Database > StorageBuilder < Db > { # [doc = " Set a callback for salsa events."] # [doc = ""] # [doc = " The `event_callback` function will be invoked by the salsa runtime at various points during execution."] pub fn event_callback (mut self , callback : Box < dyn Fn (crate :: Event) + Send + Sync + 'static > ,) -> Self { self . event_callback = Some (callback) ; self } # [doc = " Manually register an ingredient."] # [doc = ""] # [doc = " Manual ingredient registration is necessary when the `inventory` feature is disabled."] pub fn ingredient < I : HasJar > (mut self) -> Self { self . jars . push (ErasedJar :: erase :: < I > ()) ; self } # [doc = " Construct the [`Storage`] using the provided builder options."] pub fn build (self) -> Storage < Db > { Storage { handle : StorageHandle :: with_jars (self . event_callback , self . jars) , zalsa_local : ZalsaLocal :: new () , } } }
};
}
