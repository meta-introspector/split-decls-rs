// Generated macro for impl_287 (impl)
macro_rules! Depcrate_loom_std_parking_lotimpl_287 {
() => {
// Module: crate::loom::std::parking_lot
// Provides: {"impl_287"}
// Dependencies: {}
impl < T > RwLock < T > { pub (crate) fn new (t : T) -> RwLock < T > { RwLock (PhantomData , parking_lot :: RwLock :: new (t)) } pub (crate) fn read (& self) -> RwLockReadGuard < '_ , T > { RwLockReadGuard (PhantomData , self . 1 . read ()) } pub (crate) fn try_read (& self) -> Option < RwLockReadGuard < '_ , T > > { self . 1 . try_read () . map (| guard | RwLockReadGuard (PhantomData , guard)) } pub (crate) fn write (& self) -> RwLockWriteGuard < '_ , T > { RwLockWriteGuard (PhantomData , self . 1 . write ()) } pub (crate) fn try_write (& self) -> Option < RwLockWriteGuard < '_ , T > > { self . 1 . try_write () . map (| guard | RwLockWriteGuard (PhantomData , guard)) } }
};
}
