// Generated macro for EyepatchHackVector (struct)
macro_rules! Depcrate_zerovecEyepatchHackVector {
() => {
// Module: crate::zerovec
// Provides: {"EyepatchHackVector"}
// Dependencies: {}
struct EyepatchHackVector < U > { # [doc = " Pointer to data"] # [doc = " This pointer is *always* valid, the reason it is represented as a raw pointer"] # [doc = " is that it may logically represent an `&[T::ULE]` or the ptr,len of a `Vec<T::ULE>`"] buf : NonNull < [U] > , # [cfg (feature = "alloc")] # [doc = " Borrowed if zero. Capacity of buffer above if not"] capacity : usize , }
};
}
