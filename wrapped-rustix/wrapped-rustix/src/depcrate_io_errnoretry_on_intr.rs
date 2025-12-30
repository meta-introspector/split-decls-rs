// Generated macro for retry_on_intr (function)
macro_rules! Depcrate_io_errnoretry_on_intr {
() => {
// Module: crate::io::errno
// Provides: {"retry_on_intr"}
// Dependencies: {}
# [doc = " Call `f` until it either succeeds or fails other than [`Errno::INTR`]."] # [inline] pub fn retry_on_intr < T , F : FnMut () -> Result < T > > (mut f : F) -> Result < T > { loop { match f () { Err (Errno :: INTR) => () , result => return result , } } }
};
}
