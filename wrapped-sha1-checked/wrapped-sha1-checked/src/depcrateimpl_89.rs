// Generated macro for impl_89 (impl)
macro_rules! Depcrateimpl_89 {
() => {
// Module: crate
// Provides: {"impl_89"}
// Dependencies: {}
impl CollisionResult { # [doc = " Returns the output hash."] pub fn hash (& self) -> & Output < Sha1 > { match self { CollisionResult :: Ok (s) => s , CollisionResult :: Mitigated (s) => s , CollisionResult :: Collision (s) => s , } } # [doc = " Returns if there was a collision"] pub fn has_collision (& self) -> bool { ! matches ! (self , CollisionResult :: Ok (_)) } }
};
}
