// Generated macro for PrivateIterMut (struct)
macro_rules! Depcrate_punctuatedPrivateIterMut {
() => {
// Module: crate::punctuated
// Provides: {"PrivateIterMut"}
// Dependencies: {}
struct PrivateIterMut < 'a , T : 'a , P : 'a > { inner : slice :: IterMut < 'a , (T , P) > , last : option :: IntoIter < & 'a mut T > , }
};
}
