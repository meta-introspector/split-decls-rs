// Generated macro for impl_293 (impl)
macro_rules! Depcrate_typesimpl_293 {
() => {
// Module: crate::types
// Provides: {"impl_293"}
// Dependencies: {}
impl Ord for DateTime { fn cmp (& self , other : & Self) -> Ordering { if let ord @ (Ordering :: Less | Ordering :: Greater) = self . year () . cmp (& other . year ()) { return ord ; } if let ord @ (Ordering :: Less | Ordering :: Greater) = self . month () . cmp (& other . month ()) { return ord ; } if let ord @ (Ordering :: Less | Ordering :: Greater) = self . day () . cmp (& other . day ()) { return ord ; } if let ord @ (Ordering :: Less | Ordering :: Greater) = self . hour () . cmp (& other . hour ()) { return ord ; } if let ord @ (Ordering :: Less | Ordering :: Greater) = self . minute () . cmp (& other . minute ()) { return ord ; } self . second () . cmp (& other . second ()) } }
};
}
