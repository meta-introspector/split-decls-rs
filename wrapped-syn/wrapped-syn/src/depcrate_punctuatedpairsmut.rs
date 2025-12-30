// Generated macro for PairsMut (struct)
macro_rules! Depcrate_punctuatedPairsMut {
() => {
// Module: crate::punctuated
// Provides: {"PairsMut"}
// Dependencies: {}
# [doc = " An iterator over mutably borrowed pairs of type `Pair<&mut T, &mut P>`."] # [doc = ""] # [doc = " Refer to the [module documentation] for details about punctuated sequences."] # [doc = ""] # [doc = " [module documentation]: self"] pub struct PairsMut < 'a , T : 'a , P : 'a > { inner : slice :: IterMut < 'a , (T , P) > , last : option :: IntoIter < & 'a mut T > , }
};
}
