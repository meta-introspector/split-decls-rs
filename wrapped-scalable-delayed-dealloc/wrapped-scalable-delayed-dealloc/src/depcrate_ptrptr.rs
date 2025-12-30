// Generated macro for Ptr (struct)
macro_rules! Depcrate_ptrPtr {
() => {
// Module: crate::ptr
// Provides: {"Ptr"}
// Dependencies: {}
# [doc = " [`Ptr`] points to an instance."] # [derive (Debug)] pub struct Ptr < 'g , T > { instance_ptr : * const RefCounted < T > , _phantom : PhantomData < & 'g T > , }
};
}
