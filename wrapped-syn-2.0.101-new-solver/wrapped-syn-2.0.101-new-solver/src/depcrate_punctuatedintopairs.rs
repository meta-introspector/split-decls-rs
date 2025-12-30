// Generated macro for IntoPairs (struct)
macro_rules! Depcrate_punctuatedIntoPairs {
() => {
// Module: crate::punctuated
// Provides: {"IntoPairs"}
// Dependencies: {}
# [doc = " An iterator over owned pairs of type `Pair<T, P>`."] # [doc = ""] # [doc = " Refer to the [module documentation] for details about punctuated sequences."] # [doc = ""] # [doc = " [module documentation]: self"] pub struct IntoPairs < T , P > { inner : vec :: IntoIter < (T , P) > , last : option :: IntoIter < T > , }
};
}
