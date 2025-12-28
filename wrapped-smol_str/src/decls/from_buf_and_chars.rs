macro_rules! deps {
    () => {
        Repr!();
        SmolStr!();
        InlineSize!();
    };
}

macro_rules! from_buf_and_chars {
    () => {
        deps!();
        fn from_buf_and_chars (mut buf : [u8 ; INLINE_CAP] , buf_len : usize , mut iter : impl Iterator < Item = char > ,) -> SmolStr { let min_size = iter . size_hint () . 0 + buf_len ; if min_size > INLINE_CAP { let heap : String = core :: str :: from_utf8 (& buf [.. buf_len]) . unwrap () . chars () . chain (iter) . collect () ; if heap . len () <= INLINE_CAP { return SmolStr :: new_inline (& heap) ; } return SmolStr (Repr :: Heap (heap . into_boxed_str () . into ())) ; } let mut len = buf_len ; while let Some (ch) = iter . next () { let size = ch . len_utf8 () ; if size + len > INLINE_CAP { let (min_remaining , _) = iter . size_hint () ; let mut heap = String :: with_capacity (size + len + min_remaining) ; heap . push_str (core :: str :: from_utf8 (& buf [.. len]) . unwrap ()) ; heap . push (ch) ; heap . extend (iter) ; return SmolStr (Repr :: Heap (heap . into_boxed_str () . into ())) ; } ch . encode_utf8 (& mut buf [len ..]) ; len += size ; } SmolStr (Repr :: Inline { len : unsafe { InlineSize :: transmute_from_u8 (len as u8) } , buf , }) }
    };
}

from_buf_and_chars!()