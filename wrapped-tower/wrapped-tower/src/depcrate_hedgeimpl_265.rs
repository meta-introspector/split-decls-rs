// Generated macro for impl_265 (impl)
macro_rules! Depcrate_hedgeimpl_265 {
() => {
// Module: crate::hedge
// Provides: {"impl_265"}
// Dependencies: {}
impl < P , Request > select :: Policy < Request > for SelectPolicy < P > where P : Policy < Request > , { fn clone_request (& self , req : & Request) -> Option < Request > { self . policy . clone_request (req) . filter (| _ | { let mut locked = self . histo . lock () . unwrap () ; locked . read () . len () >= self . min_data_points }) } }
};
}
