// Generated macro for ChangesIter (struct)
macro_rules! Depcrate_iterChangesIter {
() => {
// Module: crate::iter
// Provides: {"ChangesIter"}
// Dependencies: {}
# [doc = " Iterator for [`DiffOp::iter_changes`]."] pub struct ChangesIter < 'lookup , Old : ? Sized , New : ? Sized , T > { old : & 'lookup Old , new : & 'lookup New , old_range : Range < usize > , new_range : Range < usize > , old_index : usize , new_index : usize , old_i : usize , new_i : usize , tag : DiffTag , _marker : PhantomData < T > , }
};
}
