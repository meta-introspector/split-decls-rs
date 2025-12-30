// Generated macro for allocate_bucket (function)
macro_rules! Depcrateallocate_bucket {
() => {
// Module: crate
// Provides: {"allocate_bucket"}
// Dependencies: {}
fn allocate_bucket < T > (size : usize) -> * mut Entry < T > { Box :: into_raw ((0 .. size) . map (| _ | Entry :: < T > { present : AtomicBool :: new (false) , value : UnsafeCell :: new (MaybeUninit :: uninit ()) , }) . collect () ,) as * mut _ }
};
}
