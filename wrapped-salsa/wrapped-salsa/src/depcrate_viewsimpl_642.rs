// Generated macro for impl_642 (impl)
macro_rules! Depcrate_viewsimpl_642 {
() => {
// Module: crate::views
// Provides: {"impl_642"}
// Dependencies: {}
impl < DbView : ? Sized + Any > DatabaseDownCaster < DbView > { # [doc = " Downcast `db` to `DbView`."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The caller must ensure that `db` is of the correct type."] # [inline] pub unsafe fn downcast_unchecked < 'db > (& self , db : RawDatabase < 'db >) -> & 'db DbView { unsafe { (self . unerased_downcaster ()) (db) . as_ref () } } # [doc = " Downcast `db` to `DbView`."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The caller must ensure that `db` is of the correct type."] # [inline] pub unsafe fn downcast_mut_unchecked < 'db > (& self , db : RawDatabase < 'db >) -> & 'db mut DbView { unsafe { (self . unerased_downcaster ()) (db) . as_mut () } } # [inline] fn unerased_downcaster (& self) -> DatabaseDownCasterSig < DbView > { unsafe { mem :: transmute :: < ErasedDatabaseDownCasterSig , DatabaseDownCasterSig < DbView > > (self . 0 . cast ,) } } }
};
}
