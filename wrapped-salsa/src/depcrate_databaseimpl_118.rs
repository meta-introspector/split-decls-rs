// Generated macro for impl_118 (impl)
macro_rules! Depcrate_databaseimpl_118 {
() => {
// Module: crate::database
// Provides: {"impl_118"}
// Dependencies: {}
impl < 'db , Db : Database + ? Sized > From < & 'db mut Db > for RawDatabase < 'db > { # [inline] fn from (db : & 'db mut Db) -> Self { RawDatabase { ptr : NonNull :: from (db) . cast () , _marker : std :: marker :: PhantomData , } } }
};
}
