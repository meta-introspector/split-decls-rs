macro_rules! deps {
    () => {
        StaticTreeDesc!();
    };
}

macro_rules! impl_176 {
    () => {
        deps!();
        impl StaticTreeDesc { const EMPTY : Self = Self { static_tree : & [] , extra_bits : & [] , extra_base : 0 , elems : 0 , max_length : 0 , } ; # [doc = " extra bits for each length code"] const EXTRA_LBITS : [u8 ; LENGTH_CODES] = [0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 1 , 1 , 1 , 1 , 2 , 2 , 2 , 2 , 3 , 3 , 3 , 3 , 4 , 4 , 4 , 4 , 5 , 5 , 5 , 5 , 0 ,] ; # [doc = " extra bits for each distance code"] const EXTRA_DBITS : [u8 ; D_CODES] = [0 , 0 , 0 , 0 , 1 , 1 , 2 , 2 , 3 , 3 , 4 , 4 , 5 , 5 , 6 , 6 , 7 , 7 , 8 , 8 , 9 , 9 , 10 , 10 , 11 , 11 , 12 , 12 , 13 , 13 ,] ; # [doc = " extra bits for each bit length code"] const EXTRA_BLBITS : [u8 ; BL_CODES] = [0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 2 , 3 , 7] ; # [doc = " The lengths of the bit length codes are sent in order of decreasing"] # [doc = " probability, to avoid transmitting the lengths for unused bit length codes."] const BL_ORDER : [u8 ; BL_CODES] = [16 , 17 , 18 , 0 , 8 , 7 , 9 , 6 , 10 , 5 , 11 , 4 , 12 , 3 , 13 , 2 , 14 , 1 , 15 ,] ; pub (crate) const L : Self = Self { static_tree : & self :: trees_tbl :: STATIC_LTREE , extra_bits : & Self :: EXTRA_LBITS , extra_base : LITERALS + 1 , elems : L_CODES , max_length : MAX_BITS as u16 , } ; pub (crate) const D : Self = Self { static_tree : & self :: trees_tbl :: STATIC_DTREE , extra_bits : & Self :: EXTRA_DBITS , extra_base : 0 , elems : D_CODES , max_length : MAX_BITS as u16 , } ; pub (crate) const BL : Self = Self { static_tree : & [] , extra_bits : & Self :: EXTRA_BLBITS , extra_base : 0 , elems : BL_CODES , max_length : MAX_BL_BITS as u16 , } ; }
    };
}

impl_176!()