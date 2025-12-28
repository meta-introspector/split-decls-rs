macro_rules! deps {
    () => {
        TransmuteRefDst!();
        BecauseMutationCompatible!();
        Wrap!();
        Immutable!();
        Valid!();
        KnownLayout!();
        IntoBytes!();
        FromBytes!();
        BecauseImmutable!();
        PointerMetadata!();
        Initialized!();
    };
}

macro_rules! impl_58 {
    () => {
        deps!();
        impl < 'a , Src : ? Sized , Dst : ? Sized > TransmuteRefDst < 'a > for Wrap < & 'a Src , & 'a Dst > where Src : KnownLayout < PointerMetadata = usize > + IntoBytes + Immutable , Dst : KnownLayout < PointerMetadata = usize > + FromBytes + Immutable , { type Dst = Dst ; # [inline (always)] fn transmute_ref (self) -> & 'a Dst { static_assert ! (Src : ? Sized + KnownLayout , Dst : ? Sized + KnownLayout => { Src :: LAYOUT . align . get () >= Dst :: LAYOUT . align . get () } , "cannot transmute reference when destination type has higher alignment than source type") ; unsafe { unsafe_with_size_eq ! (< S < Src >, D < Dst >> { let ptr = Ptr :: from_ref (self . 0) . transmute ::< S < Src >, invariant :: Valid , BecauseImmutable > () . recall_validity ::< invariant :: Initialized , _ > () . transmute ::< D < Dst >, invariant :: Initialized , (crate :: pointer :: BecauseMutationCompatible , _) > () . recall_validity ::< invariant :: Valid , _ > () ; # [allow (unused_unsafe)] let ptr = unsafe { ptr . assume_alignment () } ; & ptr . as_ref () . 0 }) } } }
    };
}

impl_58!()