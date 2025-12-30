// Generated macro for Scope (struct)
macro_rules! Depcrate_thread_scopedScope {
() => {
// Module: crate::thread::scoped
// Provides: {"Scope"}
// Dependencies: {}
# [doc = " A scope to spawn scoped threads in."] # [doc = ""] # [doc = " See [`scope`] for details."] # [stable (feature = "scoped_threads" , since = "1.63.0")] pub struct Scope < 'scope , 'env : 'scope > { data : Arc < ScopeData > , # [doc = " Invariance over 'scope, to make sure 'scope cannot shrink,"] # [doc = " which is necessary for soundness."] # [doc = ""] # [doc = " Without invariance, this would compile fine but be unsound:"] # [doc = ""] # [doc = " ```compile_fail,E0373"] # [doc = " std::thread::scope(|s| {"] # [doc = "     s.spawn(|| {"] # [doc = "         let a = String::from(\"abcd\");"] # [doc = "         s.spawn(|| println!(\"{a:?}\")); // might run after `a` is dropped"] # [doc = "     });"] # [doc = " });"] # [doc = " ```"] scope : PhantomData < & 'scope mut & 'scope () > , env : PhantomData < & 'env mut & 'env () > , }
};
}
