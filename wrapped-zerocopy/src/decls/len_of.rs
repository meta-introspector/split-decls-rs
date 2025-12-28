macro_rules! deps {
    () => {
        DstLayout!();
        SizeInfo!();
        PointerMetadata!();
        CastType!();
        MetadataCastError!();
        KnownLayout!();
    };
}

macro_rules! len_of {
    () => {
        deps!();
        mod len_of { use super :: * ; # [doc = " A witness type for metadata of a valid instance of `&T`."] pub (crate) struct MetadataOf < T : ? Sized + KnownLayout > { # [doc = " # Safety"] # [doc = ""] # [doc = " The size of an instance of `&T` with the given metadata is not"] # [doc = " larger than `isize::MAX`."] meta : T :: PointerMetadata , _p : PhantomData < T > , } impl < T : ? Sized + KnownLayout > Copy for MetadataOf < T > { } impl < T : ? Sized + KnownLayout > Clone for MetadataOf < T > { fn clone (& self) -> Self { * self } } impl < T : ? Sized > MetadataOf < T > where T : KnownLayout , { # [doc = " Returns `None` if `meta` is greater than `t`'s metadata."] # [inline (always)] pub (crate) fn new_in_bounds (t : & T , meta : usize) -> Option < Self > where T : KnownLayout < PointerMetadata = usize > , { if meta <= Ptr :: from_ref (t) . len () { Some (unsafe { Self :: new_unchecked (meta) }) } else { None } } # [doc = " # Safety"] # [doc = ""] # [doc = " The size of an instance of `&T` with the given metadata is not"] # [doc = " larger than `isize::MAX`."] pub (crate) unsafe fn new_unchecked (meta : T :: PointerMetadata) -> Self { Self { meta , _p : PhantomData } } pub (crate) fn get (& self) -> T :: PointerMetadata where T :: PointerMetadata : Copy , { self . meta } # [inline] pub (crate) fn padding_needed_for (& self) -> usize where T : KnownLayout < PointerMetadata = usize > , { let trailing_slice_layout = crate :: trailing_slice_layout :: < T > () ; # [allow (unstable_name_collisions , clippy :: incompatible_msrv)] let unpadded_size = unsafe { let trailing_size = self . meta . unchecked_mul (trailing_slice_layout . elem_size) ; trailing_size . unchecked_add (trailing_slice_layout . offset) } ; util :: padding_needed_for (unpadded_size , T :: LAYOUT . align) } # [inline (always)] pub (crate) fn validate_cast_and_convert_metadata (addr : usize , bytes_len : MetadataOf < [u8] > , cast_type : CastType , meta : Option < T :: PointerMetadata > ,) -> Result < (MetadataOf < T > , MetadataOf < [u8] >) , MetadataCastError > { let layout = match meta { None => T :: LAYOUT , Some (meta) => { let size = match T :: size_for_metadata (meta) { Some (size) => size , None => return Err (MetadataCastError :: Size) , } ; DstLayout { align : T :: LAYOUT . align , size_info : crate :: SizeInfo :: Sized { size } , statically_shallow_unpadded : false , } } } ; let (elems , split_at) = layout . validate_cast_and_convert_metadata (addr , bytes_len . get () , cast_type) ? ; let elems = T :: PointerMetadata :: from_elem_count (elems) ; let elems = meta . unwrap_or (elems) ; let elems = unsafe { MetadataOf :: new_unchecked (elems) } ; let split_at = unsafe { MetadataOf :: < [u8] > :: new_unchecked (split_at) } ; Ok ((elems , split_at)) } } }
    };
}

len_of!();