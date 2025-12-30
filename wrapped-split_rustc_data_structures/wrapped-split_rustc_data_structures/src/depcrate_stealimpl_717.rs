// Generated macro for impl_717 (impl)
macro_rules! Depcrate_stealimpl_717 {
() => {
// Module: crate::steal
// Provides: {"impl_717"}
// Dependencies: {}
impl < T > Steal < T > { pub fn new (value : T) -> Self { Steal { value : RwLock :: new (Some (value)) } } # [track_caller] pub fn borrow (& self) -> MappedReadGuard < '_ , T > { let borrow = self . value . borrow () ; if borrow . is_none () { panic ! ("attempted to read from stolen value: {}" , std :: any :: type_name ::< T > ()) ; } ReadGuard :: map (borrow , | opt | opt . as_ref () . unwrap ()) } # [track_caller] pub fn get_mut (& mut self) -> & mut T { self . value . get_mut () . as_mut () . expect ("attempt to read from stolen value") } # [track_caller] pub fn steal (& self) -> T { let value_ref = & mut * self . value . try_write () . expect ("stealing value which is locked") ; let value = value_ref . take () ; value . expect ("attempt to steal from stolen value") } # [doc = " Writers of rustc drivers often encounter stealing issues. This function makes it possible to"] # [doc = " handle these errors gracefully."] # [doc = ""] # [doc = " This should not be used within rustc as it leaks information not tracked"] # [doc = " by the query system, breaking incremental compilation."] # [rustc_lint_untracked_query_information] pub fn is_stolen (& self) -> bool { self . value . borrow () . is_none () } }
};
}
