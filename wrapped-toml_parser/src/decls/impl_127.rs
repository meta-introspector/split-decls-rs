macro_rules! deps {
    () => {
        Lexer!();
        Stream!();
        Token!();
    };
}

macro_rules! impl_127 {
    () => {
        deps!();
        impl < 'i > Lexer < 'i > { pub (crate) fn new (input : & 'i str) -> Self { let mut stream = Stream :: new (input) ; if input . as_bytes () . starts_with (BOM) { let offset = BOM . len () ; # [cfg (feature = "unsafe")] unsafe { stream . next_slice_unchecked (offset) } ; # [cfg (not (feature = "unsafe"))] stream . next_slice (offset) ; } Lexer { stream , eof : false } } # [cfg (feature = "alloc")] pub fn into_vec (self) -> Vec < Token > { # ! [allow (unused_qualifications)] let capacity = core :: cmp :: min (self . stream . len () , usize :: MAX / core :: mem :: size_of :: < Token > () ,) ; let mut vec = Vec :: with_capacity (capacity) ; vec . extend (self) ; vec } }
    };
}

impl_127!()