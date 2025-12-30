// Generated macro for impl_67 (impl)
macro_rules! Depcrate_threadimpl_67 {
() => {
// Module: crate::thread
// Provides: {"impl_67"}
// Dependencies: {}
impl Builder { # [must_use] pub fn new (intent : ThreadIntent , name : impl Into < String >) -> Self { Self { intent , inner : jod_thread :: Builder :: new () . name (name . into ()) , allow_leak : false } } # [must_use] pub fn stack_size (self , size : usize) -> Self { Self { inner : self . inner . stack_size (size) , .. self } } # [doc = " Whether dropping should detach the thread"] # [doc = " instead of joining it."] # [must_use] pub fn allow_leak (self , allow_leak : bool) -> Self { Self { allow_leak , .. self } } pub fn spawn < F , T > (self , f : F) -> std :: io :: Result < JoinHandle < T > > where F : (FnOnce () -> T) + Send + 'static , T : Send + 'static , { let inner_handle = self . inner . spawn (move | | { self . intent . apply_to_current_thread () ; f () }) ? ; Ok (JoinHandle { inner : Some (inner_handle) , allow_leak : self . allow_leak }) } }
};
}
