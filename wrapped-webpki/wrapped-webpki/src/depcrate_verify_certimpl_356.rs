// Generated macro for impl_356 (impl)
macro_rules! Depcrate_verify_certimpl_356 {
() => {
// Module: crate::verify_cert
// Provides: {"impl_356"}
// Dependencies: {}
impl < 'a > PartialPath < 'a > { pub (crate) fn new (end_entity : & 'a EndEntityCert < 'a >) -> Self { Self { end_entity , intermediates : Default :: default () , used : 0 , } } pub (crate) fn push (& mut self , cert : Cert < 'a >) -> Result < () , ControlFlow < Error , Error > > { if self . used >= MAX_SUB_CA_COUNT { return Err (Error :: MaximumPathDepthExceeded . into ()) ; } self . intermediates [self . used] = Some (cert) ; self . used += 1 ; Ok (()) } fn pop (& mut self) { debug_assert ! (self . used > 0) ; if self . used == 0 { return ; } self . used -= 1 ; self . intermediates [self . used] = None ; } pub (crate) fn node (& self) -> PathNode < '_ > { PathNode { path : self , index : self . used , cert : self . head () , } } # [doc = " Current head of the path."] pub (crate) fn head (& self) -> & Cert < 'a > { self . get (self . used) } # [doc = " Get the certificate at index `idx` in the path."] # [doc = ""] fn get (& self , idx : usize) -> & Cert < 'a > { match idx { 0 => self . end_entity , _ => self . intermediates [idx - 1] . as_ref () . unwrap () , } } }
};
}
