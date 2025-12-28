macro_rules! util {
    () => {
        pub mod util { # [inline] pub fn bsearch_range_table (c : char , r : & [(char , char)]) -> bool { use core :: cmp :: Ordering :: { Equal , Less , Greater } ; r . binary_search_by (| & (lo , hi) | { if lo <= c && c <= hi { Equal } else if hi < c { Less } else { Greater } }) . is_ok () } # [inline] fn is_alphabetic (c : char) -> bool { if super :: UNICODE_VERSION_U8 == char :: UNICODE_VERSION { c . is_alphabetic () } else { match c { 'a' ..= 'z' | 'A' ..= 'Z' => true , c if c > '\x7f' => super :: derived_property :: Alphabetic (c) , _ => false , } } } # [inline] fn is_numeric (c : char) -> bool { if super :: UNICODE_VERSION_U8 == char :: UNICODE_VERSION { c . is_numeric () } else { match c { '0' ..= '9' => true , c if c > '\x7f' => super :: general_category :: N (c) , _ => false , } } } # [inline] pub fn is_alphanumeric (c : char) -> bool { is_alphabetic (c) || is_numeric (c) } }
    };
}

util!();