// Generated macro for Locked (struct)
macro_rules! DepcrateLocked {
() => {
// Module: crate
// Provides: {"Locked"}
// Dependencies: {}
# [doc = " A locked value acquired from a `TryLock`."] # [doc = ""] # [doc = " The type represents an exclusive view at the underlying value. The lock is"] # [doc = " released when this type is dropped."] # [doc = ""] # [doc = " This type derefs to the underlying value."] # [must_use = "TryLock will immediately unlock if not used"] pub struct Locked < 'a , T : 'a > { lock : & 'a TryLock < T > , order : Ordering , # [doc = " Suppresses Send and Sync autotraits for `struct Locked`."] _p : PhantomData < * mut T > , }
};
}
