macro_rules! deps {
    () => {
        PointerMetadata!();
        KnownLayout!();
    };
}

macro_rules! impl_307 {
    () => {
        deps!();
        # [allow (clippy :: needless_lifetimes)] impl < 'a , T > PtrInner < 'a , T > where T : ? Sized + KnownLayout , { # [doc = " Extracts the metadata of this `ptr`."] pub (crate) fn meta (self) -> MetadataOf < T > { let meta = T :: pointer_to_metadata (self . as_non_null () . as_ptr ()) ; unsafe { MetadataOf :: new_unchecked (meta) } } # [doc = " Produces a `PtrInner` with the same address and provenance as `self` but"] # [doc = " the given `meta`."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The caller promises that if `self`'s referent is not zero sized, then"] # [doc = " a pointer constructed from its address with the given `meta` metadata"] # [doc = " will address a subset of the allocation pointed to by `self`."] # [inline] pub (crate) unsafe fn with_meta (self , meta : T :: PointerMetadata) -> Self where T : KnownLayout , { let raw = T :: raw_from_ptr_len (self . as_non_null () . cast () , meta) ; unsafe { PtrInner :: new (raw) } } pub (crate) fn as_bytes (self) -> PtrInner < 'a , [u8] > { let ptr = self . as_non_null () ; let bytes = match T :: size_of_val_raw (ptr) { Some (bytes) => bytes , None => unsafe { core :: hint :: unreachable_unchecked () } , } ; let ptr = core :: ptr :: slice_from_raw_parts_mut (ptr . cast :: < u8 > () . as_ptr () , bytes) ; let ptr = unsafe { NonNull :: new_unchecked (ptr) } ; unsafe { PtrInner :: new (ptr) } } }
    };
}

impl_307!()