// Generated macro for r#impl (module)
macro_rules! Depcrate_diffr#impl {
() => {
// Module: crate::diff
// Provides: {"r#impl"}
// Dependencies: {}
# [cfg (any (not (feature = "diff") , windows))] mod r#impl { use super :: Render ; pub (crate) enum Diff { } impl Diff { pub fn compute (_expected : & str , _actual : & str) -> Option < Self > { None } pub fn iter (& self , _input : & str) -> Box < dyn Iterator < Item = Render > > { let _ = Render :: Common ; let _ = Render :: Unique ; match * self { } } } }
};
}
