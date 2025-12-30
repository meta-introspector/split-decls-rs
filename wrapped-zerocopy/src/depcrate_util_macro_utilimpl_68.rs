// Generated macro for impl_68 (impl)
macro_rules! Depcrate_util_macro_utilimpl_68 {
() => {
// Module: crate::util::macro_util
// Provides: {"impl_68"}
// Dependencies: {}
impl < 'a , Src : ? Sized , Dst : ? Sized > TransmuteMutDst < 'a > for Wrap < & 'a mut Src , & 'a mut Dst > where Src : KnownLayout < PointerMetadata = usize > + FromBytes + IntoBytes , Dst : KnownLayout < PointerMetadata = usize > + FromBytes + IntoBytes , { type Dst = Dst ; # [inline (always)] fn transmute_mut (self) -> & 'a mut Dst { static_assert ! (Src : ? Sized + KnownLayout , Dst : ? Sized + KnownLayout => { Src :: LAYOUT . align . get () >= Dst :: LAYOUT . align . get () } , "cannot transmute reference when destination type has higher alignment than source type") ; unsafe { unsafe_with_size_eq ! (< S < Src >, D < Dst >> { let ptr = Ptr :: from_mut (self . 0) . transmute ::< S < Src >, invariant :: Valid , _ > () . recall_validity ::< invariant :: Initialized , (_ , (_ , _)) > () . transmute ::< D < Dst >, invariant :: Initialized , _ > () . recall_validity ::< invariant :: Valid , (_ , (_ , _)) > () ; # [allow (unused_unsafe)] let ptr = unsafe { ptr . assume_alignment () } ; & mut ptr . as_mut () . 0 }) } } }
};
}
