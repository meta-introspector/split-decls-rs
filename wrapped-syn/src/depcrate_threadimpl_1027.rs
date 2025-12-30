// Generated macro for impl_1027 (impl)
macro_rules! Depcrate_threadimpl_1027 {
() => {
// Module: crate::thread
// Provides: {"impl_1027"}
// Dependencies: {}
impl < T : Debug > Debug for ThreadBound < T > { fn fmt (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { match self . get () { Some (value) => Debug :: fmt (value , formatter) , None => formatter . write_str ("unknown") , } } }
};
}
