// Generated macro for impl_21 (impl)
macro_rules! Depcrate_attrimpl_21 {
() => {
// Module: crate::attr
// Provides: {"impl_21"}
// Dependencies: {}
impl Context { pub (crate) fn error (& self , e : Error) { match self . error . borrow_mut () . as_mut () . unwrap () { Some (base) => base . combine (e) , error @ None => * error = Some (e) , } } pub (crate) fn check (self) -> Result < () , Error > { match self . error . borrow_mut () . take () . unwrap () { Some (e) => Err (e) , None => Ok (()) , } } }
};
}
