// Generated macro for borrowed_lifetimes (function)
macro_rules! Depcrate_deborrowed_lifetimes {
() => {
// Module: crate::de
// Provides: {"borrowed_lifetimes"}
// Dependencies: {}
fn borrowed_lifetimes (cont : & Container) -> BorrowedLifetimes { let mut lifetimes = BTreeSet :: new () ; for field in cont . data . all_fields () { if ! field . attrs . skip_deserializing () { lifetimes . extend (field . attrs . borrowed_lifetimes () . iter () . cloned ()) ; } } if lifetimes . iter () . any (| b | b . to_string () == "'static") { BorrowedLifetimes :: Static } else { BorrowedLifetimes :: Borrowed (lifetimes) } }
};
}
