// Generated macro for impl_71 (impl)
macro_rules! Depcrate_convert_implsimpl_71 {
() => {
// Module: crate::convert::impls
// Provides: {"impl_71"}
// Dependencies: {}
impl < T : WasmAbi < Prim4 = () > > WasmAbi for Option < T > { # [doc = " Whether this `Option` is a `Some` value."] type Prim1 = u32 ; type Prim2 = T :: Prim1 ; type Prim3 = T :: Prim2 ; type Prim4 = T :: Prim3 ; # [inline] fn split (self) -> (u32 , T :: Prim1 , T :: Prim2 , T :: Prim3) { match self { None => (0 , Default :: default () , Default :: default () , Default :: default () ,) , Some (value) => { let (prim1 , prim2 , prim3 , ()) = value . split () ; (1 , prim1 , prim2 , prim3) } } } # [inline] fn join (is_some : u32 , prim1 : T :: Prim1 , prim2 : T :: Prim2 , prim3 : T :: Prim3) -> Self { if is_some == 0 { None } else { Some (T :: join (prim1 , prim2 , prim3 , ())) } } }
};
}
