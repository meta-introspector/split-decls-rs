macro_rules! deps {
    () => {
        Flags!();
    };
}

macro_rules! impl_234 {
    () => {
        deps!();
        impl Flags { # [doc = " set if currently processing the last block"] const IS_LAST_BLOCK : Self = Self (0b0000_0001) ; # [doc = " set if a custom dictionary was provided"] const HAVE_DICT : Self = Self (0b0000_0010) ; # [doc = " if false, allow invalid distance too far"] const SANE : Self = Self (0b0000_0100) ; pub (crate) const fn contains (self , other : Self) -> bool { debug_assert ! (other . 0 . count_ones () == 1) ; self . 0 & other . 0 != 0 } # [inline (always)] pub (crate) fn update (& mut self , other : Self , value : bool) { if value { * self = Self (self . 0 | other . 0) ; } else { * self = Self (self . 0 & ! other . 0) ; } } }
    };
}

impl_234!();