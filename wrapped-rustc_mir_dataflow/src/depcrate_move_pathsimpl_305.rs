// Generated macro for impl_305 (impl)
macro_rules! Depcrate_move_pathsimpl_305 {
() => {
// Module: crate::move_paths
// Provides: {"impl_305"}
// Dependencies: {}
impl < 'tcx > fmt :: Debug for MovePath < 'tcx > { fn fmt (& self , w : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (w , "MovePath {{") ? ; if let Some (parent) = self . parent { write ! (w , " parent: {parent:?},") ? ; } if let Some (first_child) = self . first_child { write ! (w , " first_child: {first_child:?},") ? ; } if let Some (next_sibling) = self . next_sibling { write ! (w , " next_sibling: {next_sibling:?}") ? ; } write ! (w , " place: {:?} }}" , self . place) } }
};
}
