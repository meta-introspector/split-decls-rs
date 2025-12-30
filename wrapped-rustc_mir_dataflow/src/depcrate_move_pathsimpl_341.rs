// Generated macro for impl_341 (impl)
macro_rules! Depcrate_move_pathsimpl_341 {
() => {
// Module: crate::move_paths
// Provides: {"impl_341"}
// Dependencies: {}
impl < 'tcx > MoveData < 'tcx > { pub fn gather_moves (body : & Body < 'tcx > , tcx : TyCtxt < 'tcx > , filter : impl Fn (Ty < 'tcx >) -> bool ,) -> MoveData < 'tcx > { builder :: gather_moves (body , tcx , filter) } # [doc = " For the move path `mpi`, returns the root local variable that starts the path."] # [doc = " (e.g., for a path like `a.b.c` returns `a`)"] pub fn base_local (& self , mut mpi : MovePathIndex) -> Local { loop { let path = & self . move_paths [mpi] ; if let Some (l) = path . place . as_local () { return l ; } mpi = path . parent . expect ("root move paths should be locals") ; } } pub fn find_in_move_path_or_its_descendants (& self , root : MovePathIndex , pred : impl Fn (MovePathIndex) -> bool ,) -> Option < MovePathIndex > { if pred (root) { return Some (root) ; } self . move_paths [root] . find_descendant (& self . move_paths , pred) } }
};
}
