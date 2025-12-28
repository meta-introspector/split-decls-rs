macro_rules! deps {
    () => {
        InlineSize!();
        SmolStr!();
        SmolStrBuilder!();
        StrExt!();
        Repr!();
    };
}

macro_rules! impl_54 {
    () => {
        deps!();
        impl StrExt for str { # [inline] fn to_lowercase_smolstr (& self) -> SmolStr { let len = self . len () ; if len <= INLINE_CAP { let (buf , rest) = inline_convert_while_ascii (self , u8 :: to_ascii_lowercase) ; from_buf_and_chars (buf , len - rest . len () , rest . chars () . flat_map (| c | c . to_lowercase ())) } else { self . to_lowercase () . into () } } # [inline] fn to_uppercase_smolstr (& self) -> SmolStr { let len = self . len () ; if len <= INLINE_CAP { let (buf , rest) = inline_convert_while_ascii (self , u8 :: to_ascii_uppercase) ; from_buf_and_chars (buf , len - rest . len () , rest . chars () . flat_map (| c | c . to_uppercase ())) } else { self . to_uppercase () . into () } } # [inline] fn to_ascii_lowercase_smolstr (& self) -> SmolStr { let len = self . len () ; if len <= INLINE_CAP { let mut buf = [0u8 ; INLINE_CAP] ; buf [.. len] . copy_from_slice (self . as_bytes ()) ; buf [.. len] . make_ascii_lowercase () ; SmolStr (Repr :: Inline { len : unsafe { InlineSize :: transmute_from_u8 (len as u8) } , buf , }) } else { self . to_ascii_lowercase () . into () } } # [inline] fn to_ascii_uppercase_smolstr (& self) -> SmolStr { let len = self . len () ; if len <= INLINE_CAP { let mut buf = [0u8 ; INLINE_CAP] ; buf [.. len] . copy_from_slice (self . as_bytes ()) ; buf [.. len] . make_ascii_uppercase () ; SmolStr (Repr :: Inline { len : unsafe { InlineSize :: transmute_from_u8 (len as u8) } , buf , }) } else { self . to_ascii_uppercase () . into () } } # [inline] fn replace_smolstr (& self , from : & str , to : & str) -> SmolStr { self . replacen_smolstr (from , to , usize :: MAX) } # [inline] fn replacen_smolstr (& self , from : & str , to : & str , mut count : usize) -> SmolStr { if let [from_u8] = from . as_bytes () && let [to_u8] = to . as_bytes () { return if self . len () <= count { unsafe { replacen_1_ascii (self , | b | if b == from_u8 { * to_u8 } else { * b }) } } else { unsafe { replacen_1_ascii (self , | b | { if b == from_u8 && count != 0 { count -= 1 ; * to_u8 } else { * b } }) } } ; } let mut result = SmolStrBuilder :: new () ; let mut last_end = 0 ; for (start , part) in self . match_indices (from) . take (count) { result . push_str (unsafe { self . get_unchecked (last_end .. start) }) ; result . push_str (to) ; last_end = start + part . len () ; } result . push_str (unsafe { self . get_unchecked (last_end .. self . len ()) }) ; SmolStr :: from (result) } }
    };
}

impl_54!()