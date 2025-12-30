// Generated macro for ErasedValue (struct)
macro_rules! Depcrate_anyErasedValue {
() => {
// Module: crate::any
// Provides: {"ErasedValue"}
// Dependencies: {}
pub (crate) struct ErasedValue { ptr : * mut () , drop : unsafe fn (* mut ()) , # [cfg (any (debug_assertions , miri))] type_id : TypeId , # [cfg (any (debug_assertions , miri))] type_name : & 'static str , }
};
}
