macro_rules! deps {
    () => {
        InternerInner!();
        Symbol!();
        ByteSymbol!();
        Interner!();
    };
}

macro_rules! impl_209 {
    () => {
        deps!();
        impl Interner { fn prefill (init : & [& 'static str] , extra : & [& 'static str]) -> Self { let byte_strs = FxIndexSet :: from_iter (init . iter () . copied () . chain (extra . iter () . copied ()) . map (| str | str . as_bytes ()) ,) ; # [expect (rustc :: potential_query_instability)] if byte_strs . len () != init . len () + extra . len () { panic ! ("duplicate symbols in the rustc symbol list and the extra symbols added by the driver: {:?}" , FxHashSet :: intersection (& init . iter () . copied () . collect () , & extra . iter () . copied () . collect () ,) . collect ::< Vec < _ >> ()) } Interner (Lock :: new (InternerInner { arena : Default :: default () , byte_strs })) } fn intern_str (& self , str : & str) -> Symbol { Symbol :: new (self . intern_inner (str . as_bytes ())) } fn intern_byte_str (& self , byte_str : & [u8]) -> ByteSymbol { ByteSymbol :: new (self . intern_inner (byte_str)) } # [inline] fn intern_inner (& self , byte_str : & [u8]) -> u32 { let mut inner = self . 0 . lock () ; if let Some (idx) = inner . byte_strs . get_index_of (byte_str) { return idx as u32 ; } let byte_str : & [u8] = inner . arena . alloc_slice (byte_str) ; let byte_str : & 'static [u8] = unsafe { & * (byte_str as * const [u8]) } ; let (idx , is_new) = inner . byte_strs . insert_full (byte_str) ; debug_assert ! (is_new) ; idx as u32 } # [doc = " Get the symbol as a string."] # [doc = ""] # [doc = " [`Symbol::as_str()`] should be used in preference to this function."] fn get_str (& self , symbol : Symbol) -> & str { let byte_str = self . get_inner (symbol . 0 . as_usize ()) ; unsafe { str :: from_utf8_unchecked (byte_str) } } # [doc = " Get the symbol as a string."] # [doc = ""] # [doc = " [`ByteSymbol::as_byte_str()`] should be used in preference to this function."] fn get_byte_str (& self , symbol : ByteSymbol) -> & [u8] { self . get_inner (symbol . 0 . as_usize ()) } fn get_inner (& self , index : usize) -> & [u8] { self . 0 . lock () . byte_strs . get_index (index) . unwrap () } }
    };
}

impl_209!();