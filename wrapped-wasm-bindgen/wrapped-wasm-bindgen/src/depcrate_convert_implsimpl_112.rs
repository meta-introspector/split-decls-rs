// Generated macro for impl_112 (impl)
macro_rules! Depcrate_convert_implsimpl_112 {
() => {
// Module: crate::convert::impls
// Provides: {"impl_112"}
// Dependencies: {}
impl < T : WasmAbi < Prim3 = () , Prim4 = () > > WasmAbi for Result < T , u32 > { type Prim1 = T :: Prim1 ; type Prim2 = T :: Prim2 ; # [doc = " If this `Result` is an `Err`, the error value."] type Prim3 = u32 ; # [doc = " Whether this `Result` is an `Err`."] type Prim4 = u32 ; # [inline] fn split (self) -> (T :: Prim1 , T :: Prim2 , u32 , u32) { match self { Ok (value) => { let (prim1 , prim2 , () , ()) = value . split () ; (prim1 , prim2 , 0 , 0) } Err (err) => (Default :: default () , Default :: default () , err , 1) , } } # [inline] fn join (prim1 : T :: Prim1 , prim2 : T :: Prim2 , err : u32 , is_err : u32) -> Self { if is_err == 0 { Ok (T :: join (prim1 , prim2 , () , ())) } else { Err (err) } } }
};
}
