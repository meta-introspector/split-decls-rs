// Generated macro for Tid (struct)
macro_rules! Depcrate_tidTid {
() => {
// Module: crate::tid
// Provides: {"Tid"}
// Dependencies: {}
# [doc = " Uniquely identifies a thread."] pub (crate) struct Tid < C > { id : usize , _not_send : PhantomData < UnsafeCell < () > > , _cfg : PhantomData < fn (C) > , }
};
}
