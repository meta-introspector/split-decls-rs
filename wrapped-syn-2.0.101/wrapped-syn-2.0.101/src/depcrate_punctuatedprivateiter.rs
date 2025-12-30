// Generated macro for PrivateIter (struct)
macro_rules! Depcrate_punctuatedPrivateIter {
() => {
// Module: crate::punctuated
// Provides: {"PrivateIter"}
// Dependencies: {}
struct PrivateIter < 'a , T : 'a , P : 'a > { inner : slice :: Iter < 'a , (T , P) > , last : option :: IntoIter < & 'a T > , }
};
}
