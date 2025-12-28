macro_rules! deps {
    () => {
        AsAddress!();
        MetadataCastError!();
        CastType!();
        SizeError!();
        CastError!();
        AlignmentError!();
        KnownLayout!();
        Alignment!();
        PointerMetadata!();
    };
}

macro_rules! impl_311 {
    () => {
        deps!();
        impl < 'a > PtrInner < 'a , [u8] > { # [doc = " Attempts to cast `self` to a `U` using the given cast type."] # [doc = ""] # [doc = " If `U` is a slice DST and pointer metadata (`meta`) is provided, then"] # [doc = " the cast will only succeed if it would produce an object with the given"] # [doc = " metadata."] # [doc = ""] # [doc = " Returns `None` if the resulting `U` would be invalidly-aligned, if no"] # [doc = " `U` can fit in `self`, or if the provided pointer metadata describes an"] # [doc = " invalid instance of `U`. On success, returns a pointer to the"] # [doc = " largest-possible `U` which fits in `self`."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The caller may assume that this implementation is correct, and may rely"] # [doc = " on that assumption for the soundness of their code. In particular, the"] # [doc = " caller may assume that, if `try_cast_into` returns `Some((ptr,"] # [doc = " remainder))`, then `ptr` and `remainder` refer to non-overlapping byte"] # [doc = " ranges within `self`, and that `ptr` and `remainder` entirely cover"] # [doc = " `self`. Finally:"] # [doc = " - If this is a prefix cast, `ptr` has the same address as `self`."] # [doc = " - If this is a suffix cast, `remainder` has the same address as `self`."] # [inline] pub (crate) fn try_cast_into < U > (self , cast_type : CastType , meta : Option < U :: PointerMetadata > ,) -> Result < (PtrInner < 'a , U > , PtrInner < 'a , [u8] >) , CastError < Self , U > > where U : 'a + ? Sized + KnownLayout , { let maybe_metadata = MetadataOf :: < U > :: validate_cast_and_convert_metadata (AsAddress :: addr (self . as_non_null () . as_ptr ()) , self . meta () , cast_type , meta ,) ; let (elems , split_at) = match maybe_metadata { Ok ((elems , split_at)) => (elems , split_at) , Err (MetadataCastError :: Alignment) => { let err = unsafe { AlignmentError :: < _ , U > :: new_unchecked (self) } ; return Err (CastError :: Alignment (err)) ; } Err (MetadataCastError :: Size) => return Err (CastError :: Size (SizeError :: new (self))) , } ; let (l_slice , r_slice) = unsafe { self . split_at_unchecked (split_at) } ; let (target , remainder) = match cast_type { CastType :: Prefix => (l_slice , r_slice) , CastType :: Suffix => (r_slice , l_slice) , } ; let base = target . as_non_null () . cast :: < u8 > () ; let ptr = U :: raw_from_ptr_len (base , elems . get ()) ; Ok ((unsafe { PtrInner :: new (ptr) } , remainder)) } }
    };
}

impl_311!()