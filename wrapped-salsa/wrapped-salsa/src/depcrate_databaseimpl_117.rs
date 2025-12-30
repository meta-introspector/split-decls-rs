// Generated macro for impl_117 (impl)
macro_rules! Depcrate_databaseimpl_117 {
() => {
// Module: crate::database
// Provides: {"impl_117"}
// Dependencies: {}
impl < 'db , Db : Database + ? Sized > From < & 'db Db > for RawDatabase < 'db > { # [inline] fn from (db : & 'db Db) -> Self { RawDatabase { ptr : NonNull :: from (db) . cast () , _marker : std :: marker :: PhantomData , } } }
};
}
