// Generated macro for impl_122 (impl)
macro_rules! Depcrate_errorimpl_122 {
() => {
// Module: crate::error
// Provides: {"impl_122"}
// Dependencies: {}
# [cfg (feature = "std")] impl < I , C > AddContext < I , C > for TreeError < I , C > where I : Stream + Clone , { fn add_context (self , input : & I , token_start : & < I as Stream > :: Checkpoint , context : C) -> Self { let mut input = input . clone () ; input . reset (token_start) ; let frame = TreeErrorFrame :: Context (TreeErrorContext { input , context }) ; self . append_frame (frame) } }
};
}
