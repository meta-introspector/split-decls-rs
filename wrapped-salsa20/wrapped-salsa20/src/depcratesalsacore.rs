// Generated macro for SalsaCore (struct)
macro_rules! DepcrateSalsaCore {
() => {
// Module: crate
// Provides: {"SalsaCore"}
// Dependencies: {}
# [doc = " The Salsa20 core function."] pub struct SalsaCore < R : Unsigned > { # [doc = " Internal state of the core function"] state : [u32 ; STATE_WORDS] , # [doc = " Number of rounds to perform"] rounds : PhantomData < R > , }
};
}
