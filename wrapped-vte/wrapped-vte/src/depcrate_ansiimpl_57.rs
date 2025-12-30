// Generated macro for impl_57 (impl)
macro_rules! Depcrate_ansiimpl_57 {
() => {
// Module: crate::ansi
// Provides: {"impl_57"}
// Dependencies: {}
impl < 'a , H : Handler + 'a , T : Timeout > Performer < 'a , H , T > { # [doc = " Create a performer."] # [inline] pub fn new < 'b > (state : & 'b mut ProcessorState < T > , handler : & 'b mut H) -> Performer < 'b , H , T > { Performer { state , handler , terminated : Default :: default () } } }
};
}
