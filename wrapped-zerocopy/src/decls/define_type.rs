macro_rules! deps {
    () => {
        BigEndian!();
        KnownLayout!();
        FromBytes!();
        IntoBytes!();
        Unaligned!();
        NativeEndian!();
        ByteOrder!();
        Order!();
        NetworkEndian!();
        FromZeros!();
        TryFromBytes!();
        LittleEndian!();
        Immutable!();
    };
}

macro_rules! define_type {
    () => {
        deps!();
        macro_rules ! define_type { ($ article : ident , $ description : expr , $ name : ident , $ native : ident , $ bits : expr , $ bytes : expr , $ from_be_fn : path , $ to_be_fn : path , $ from_le_fn : path , $ to_le_fn : path , $ number_kind : tt , [$ ($ larger_native : ty) ,*] , [$ ($ larger_native_try : ty) ,*] , [$ ($ larger_byteorder : ident) ,*] , [$ ($ larger_byteorder_try : ident) ,*]) => { doc_comment ! { concat ! ($ description , " stored in a given byte order.

`" , stringify ! ($ name) , "` is like the native `" , stringify ! ($ native) , "` type with
two major differences: First, it has no alignment requirement (its alignment is 1).
Second, the endianness of its memory layout is given by the type parameter `O`,
which can be any type which implements [`ByteOrder`]. In particular, this refers
to [`BigEndian`], [`LittleEndian`], [`NativeEndian`], and [`NetworkEndian`].

" , stringify ! ($ article) , " `" , stringify ! ($ name) , "` can be constructed using
the [`new`] method, and its contained value can be obtained as a native
`" , stringify ! ($ native) , "` using the [`get`] method, or updated in place with
the [`set`] method. In all cases, if the endianness `O` is not the same as the
endianness of the current platform, an endianness swap will be performed in
order to uphold the invariants that a) the layout of `" , stringify ! ($ name) , "`
has endianness `O` and that, b) the layout of `" , stringify ! ($ native) , "` has
the platform's native endianness.

`" , stringify ! ($ name) , "` implements [`FromBytes`], [`IntoBytes`], and [`Unaligned`],
making it useful for parsing and serialization. See the module documentation for an
example of how it can be used for parsing UDP packets.

[`new`]: crate::byteorder::" , stringify ! ($ name) , "::new
[`get`]: crate::byteorder::" , stringify ! ($ name) , "::get
[`set`]: crate::byteorder::" , stringify ! ($ name) , "::set
[`FromBytes`]: crate::FromBytes
[`IntoBytes`]: crate::IntoBytes
[`Unaligned`]: crate::Unaligned") , # [derive (Copy , Clone , Eq , PartialEq , Hash)] # [cfg_attr (any (feature = "derive" , test) , derive (KnownLayout , Immutable , FromBytes , IntoBytes , Unaligned))] # [repr (transparent)] pub struct $ name < O > ([u8 ; $ bytes] , PhantomData < O >) ; } # [cfg (not (any (feature = "derive" , test)))] impl_known_layout ! (O => $ name < O >) ; # [allow (unused_unsafe)] const _ : () = unsafe { impl_or_verify ! (O => Immutable for $ name < O >) ; impl_or_verify ! (O => TryFromBytes for $ name < O >) ; impl_or_verify ! (O => FromZeros for $ name < O >) ; impl_or_verify ! (O => FromBytes for $ name < O >) ; impl_or_verify ! (O => IntoBytes for $ name < O >) ; impl_or_verify ! (O => Unaligned for $ name < O >) ; } ; impl < O > Default for $ name < O > { # [inline (always)] fn default () -> $ name < O > { $ name :: ZERO } } impl < O > $ name < O > { # [doc = " The value zero."] # [doc = ""] # [doc = " This constant should be preferred to constructing a new value"] # [doc = " using `new`, as `new` may perform an endianness swap depending"] # [doc = " on the endianness and platform."] pub const ZERO : $ name < O > = $ name ([0u8 ; $ bytes] , PhantomData) ; define_max_value_constant ! ($ name , $ bytes , $ number_kind) ; # [doc = " Constructs a new value from bytes which are already in `O` byte"] # [doc = " order."] # [must_use = "has no side effects"] # [inline (always)] pub const fn from_bytes (bytes : [u8 ; $ bytes]) -> $ name < O > { $ name (bytes , PhantomData) } # [doc = " Extracts the bytes of `self` without swapping the byte order."] # [doc = ""] # [doc = " The returned bytes will be in `O` byte order."] # [must_use = "has no side effects"] # [inline (always)] pub const fn to_bytes (self) -> [u8 ; $ bytes] { self . 0 } } impl < O : ByteOrder > $ name < O > { maybe_const_trait_bounded_fn ! { # [doc = " Constructs a new value, possibly performing an endianness"] # [doc = " swap to guarantee that the returned value has endianness"] # [doc = " `O`."] # [must_use = "has no side effects"] # [inline (always)] pub const fn new (n : $ native) -> $ name < O > { let bytes = match O :: ORDER { Order :: BigEndian => $ to_be_fn (n) , Order :: LittleEndian => $ to_le_fn (n) , } ; $ name (bytes , PhantomData) } } maybe_const_trait_bounded_fn ! { # [doc = " Returns the value as a primitive type, possibly performing"] # [doc = " an endianness swap to guarantee that the return value has"] # [doc = " the endianness of the native platform."] # [must_use = "has no side effects"] # [inline (always)] pub const fn get (self) -> $ native { match O :: ORDER { Order :: BigEndian => $ from_be_fn (self . 0) , Order :: LittleEndian => $ from_le_fn (self . 0) , } } } # [doc = " Updates the value in place as a primitive type, possibly"] # [doc = " performing an endianness swap to guarantee that the stored value"] # [doc = " has the endianness `O`."] # [inline (always)] pub fn set (& mut self , n : $ native) { * self = Self :: new (n) ; } } impl < O : ByteOrder > From <$ name < O >> for [u8 ; $ bytes] { # [inline (always)] fn from (x : $ name < O >) -> [u8 ; $ bytes] { x . 0 } } impl < O : ByteOrder > From < [u8 ; $ bytes] > for $ name < O > { # [inline (always)] fn from (bytes : [u8 ; $ bytes]) -> $ name < O > { $ name (bytes , PhantomData) } } impl < O : ByteOrder > From <$ name < O >> for $ native { # [inline (always)] fn from (x : $ name < O >) -> $ native { x . get () } } impl < O : ByteOrder > From <$ native > for $ name < O > { # [inline (always)] fn from (x : $ native) -> $ name < O > { $ name :: new (x) } } $ (impl < O : ByteOrder > From <$ name < O >> for $ larger_native { # [inline (always)] fn from (x : $ name < O >) -> $ larger_native { x . get () . into () } }) * $ (impl < O : ByteOrder > TryFrom <$ larger_native_try > for $ name < O > { type Error = TryFromIntError ; # [inline (always)] fn try_from (x : $ larger_native_try) -> Result <$ name < O >, TryFromIntError > { $ native :: try_from (x) . map ($ name :: new) } }) * $ (impl < O : ByteOrder , P : ByteOrder > From <$ name < O >> for $ larger_byteorder < P > { # [inline (always)] fn from (x : $ name < O >) -> $ larger_byteorder < P > { $ larger_byteorder :: new (x . get () . into ()) } }) * $ (impl < O : ByteOrder , P : ByteOrder > TryFrom <$ larger_byteorder_try < P >> for $ name < O > { type Error = TryFromIntError ; # [inline (always)] fn try_from (x : $ larger_byteorder_try < P >) -> Result <$ name < O >, TryFromIntError > { x . get () . try_into () . map ($ name :: new) } }) * impl < O > AsRef < [u8 ; $ bytes] > for $ name < O > { # [inline (always)] fn as_ref (& self) -> & [u8 ; $ bytes] { & self . 0 } } impl < O > AsMut < [u8 ; $ bytes] > for $ name < O > { # [inline (always)] fn as_mut (& mut self) -> & mut [u8 ; $ bytes] { & mut self . 0 } } impl < O > PartialEq <$ name < O >> for [u8 ; $ bytes] { # [inline (always)] fn eq (& self , other : &$ name < O >) -> bool { self . eq (& other . 0) } } impl < O > PartialEq < [u8 ; $ bytes] > for $ name < O > { # [inline (always)] fn eq (& self , other : & [u8 ; $ bytes]) -> bool { self . 0 . eq (other) } } impl < O : ByteOrder > PartialEq <$ native > for $ name < O > { # [inline (always)] fn eq (& self , other : &$ native) -> bool { self . get () . eq (other) } } impl_fmt_traits ! ($ name , $ native , $ number_kind) ; impl_ops_traits ! ($ name , $ native , $ number_kind) ; impl < O : ByteOrder > Debug for $ name < O > { # [inline] fn fmt (& self , f : & mut Formatter <'_ >) -> fmt :: Result { f . debug_tuple (stringify ! ($ name)) . field (& self . get ()) . finish () } } } ; }
    };
}

define_type!()