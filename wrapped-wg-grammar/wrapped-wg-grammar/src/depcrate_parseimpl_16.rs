// Generated macro for impl_16 (impl)
macro_rules! Depcrate_parseimpl_16 {
() => {
// Module: crate::parse
// Provides: {"impl_16"}
// Dependencies: {}
impl < 'a , 'i , I : :: gll :: runtime :: Input , T > From < Ambiguity < Handle < 'a , 'i , I , [T] > > > for Ambiguity < Handle < 'a , 'i , I , Any > > { fn from (x : Ambiguity < Handle < 'a , 'i , I , [T] > >) -> Self { Ambiguity (Handle { node : x . 0 . node , parser : x . 0 . parser , _marker : PhantomData , }) } }
};
}
