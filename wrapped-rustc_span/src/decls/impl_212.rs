macro_rules! deps {
    () => {
        Symbol!();
        Edition!();
    };
}

macro_rules! impl_212 {
    () => {
        deps!();
        impl Symbol { fn is_special (self) -> bool { self <= kw :: Underscore } fn is_used_keyword_always (self) -> bool { self >= kw :: As && self <= kw :: While } fn is_unused_keyword_always (self) -> bool { self >= kw :: Abstract && self <= kw :: Yield } fn is_used_keyword_conditional (self , edition : impl FnOnce () -> Edition) -> bool { (self >= kw :: Async && self <= kw :: Dyn) && edition () >= Edition :: Edition2018 } fn is_unused_keyword_conditional (self , edition : impl Copy + FnOnce () -> Edition) -> bool { self == kw :: Gen && edition () . at_least_rust_2024 () || self == kw :: Try && edition () . at_least_rust_2018 () } pub fn is_reserved (self , edition : impl Copy + FnOnce () -> Edition) -> bool { self . is_special () || self . is_used_keyword_always () || self . is_unused_keyword_always () || self . is_used_keyword_conditional (edition) || self . is_unused_keyword_conditional (edition) } pub fn is_weak (self) -> bool { self >= kw :: Auto && self <= kw :: Yeet } # [doc = " A keyword or reserved identifier that can be used as a path segment."] pub fn is_path_segment_keyword (self) -> bool { self == kw :: Super || self == kw :: SelfLower || self == kw :: SelfUpper || self == kw :: Crate || self == kw :: PathRoot || self == kw :: DollarCrate } # [doc = " Returns `true` if the symbol is `true` or `false`."] pub fn is_bool_lit (self) -> bool { self == kw :: True || self == kw :: False } # [doc = " Returns `true` if this symbol can be a raw identifier."] pub fn can_be_raw (self) -> bool { self != sym :: empty && self != kw :: Underscore && ! self . is_path_segment_keyword () } # [doc = " Was this symbol index predefined in the compiler's `symbols!` macro?"] # [doc = " Note: this applies to both `Symbol`s and `ByteSymbol`s, which is why it"] # [doc = " takes a `u32` argument instead of a `&self` argument. Use with care."] pub fn is_predefined (index : u32) -> bool { index < PREDEFINED_SYMBOLS_COUNT } }
    };
}

impl_212!();