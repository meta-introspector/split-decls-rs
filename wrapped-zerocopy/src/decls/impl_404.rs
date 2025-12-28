macro_rules! deps {
    () => {
        SplitAt!();
        Validity!();
        IntoBytes!();
        Aliasing!();
        Immutable!();
        Alignment!();
        Exclusive!();
        Valid!();
        Unaligned!();
        Split!();
        Aligned!();
        Invariants!();
        Shared!();
    };
}

macro_rules! impl_404 {
    () => {
        deps!();
        impl < 'a , T , I > Split < Ptr < 'a , T , I > > where T : ? Sized + SplitAt , I : Invariants < Alignment = Aligned , Validity = Valid > , { fn into_ref (self) -> Split < & 'a T > where I : Invariants < Aliasing = Shared > , { unsafe { Split :: new (self . source . as_ref () , self . l_len) } } fn into_mut (self) -> Split < & 'a mut T > where I : Invariants < Aliasing = Exclusive > , { unsafe { Split :: new (self . source . unify_invariants () . as_mut () , self . l_len) } } # [doc = " Produces the length of `self`'s left part."] # [inline (always)] fn l_len (& self) -> MetadataOf < T > { unsafe { MetadataOf :: < T > :: new_unchecked (self . l_len) } } # [doc = " Produces the split parts of `self`, using [`Immutable`] to ensure that"] # [doc = " it is sound to have concurrent references to both parts."] # [inline (always)] fn via_immutable (self) -> (Ptr < 'a , T , I > , Ptr < 'a , [T :: Elem] , I >) where T : Immutable , I : Invariants < Aliasing = Shared > , { unsafe { self . via_unchecked () } } # [doc = " Produces the split parts of `self`, using [`IntoBytes`] to ensure that"] # [doc = " it is sound to have concurrent references to both parts."] # [inline (always)] fn via_into_bytes (self) -> (Ptr < 'a , T , I > , Ptr < 'a , [T :: Elem] , I >) where T : IntoBytes , { unsafe { self . via_unchecked () } } # [doc = " Produces the split parts of `self`, using [`Unaligned`] to ensure that"] # [doc = " it is sound to have concurrent references to both parts."] # [inline (always)] fn via_unaligned (self) -> (Ptr < 'a , T , I > , Ptr < 'a , [T :: Elem] , I >) where T : Unaligned , { unsafe { self . via_unchecked () } } # [doc = " Produces the split parts of `self`, using a dynamic check to ensure that"] # [doc = " it is sound to have concurrent references to both parts. You should"] # [doc = " prefer using [`Self::via_immutable`], [`Self::via_into_bytes`], or"] # [doc = " [`Self::via_unaligned`], which have no runtime cost."] # [inline (always)] fn via_runtime_check (self) -> Result < (Ptr < 'a , T , I > , Ptr < 'a , [T :: Elem] , I >) , Self > { let l_len = self . l_len () ; if l_len . padding_needed_for () == 0 { Ok (unsafe { self . via_unchecked () }) } else { Err (self) } } # [doc = " Unsafely produces the split parts of `self`."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The caller promises that if `I::Aliasing` is [`Exclusive`] or `T`"] # [doc = " permits interior mutation, then `l_len.padding_needed_for() == 0`."] # [inline (always)] unsafe fn via_unchecked (self) -> (Ptr < 'a , T , I > , Ptr < 'a , [T :: Elem] , I >) { let l_len = self . l_len () ; let inner = self . source . as_inner () ; let (left , right) = unsafe { inner . split_at_unchecked (l_len) } ; let left = unsafe { Ptr :: from_inner (left) } ; let right = unsafe { Ptr :: from_inner (right) } ; (left , right) } }
    };
}

impl_404!();