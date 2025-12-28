macro_rules! deps {
    () => {
        InlineSize!();
        Repr!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        impl Repr { # [doc = " This function tries to create a new Repr::Inline or Repr::Static"] # [doc = " If it isn't possible, this function returns None"] fn new_on_stack < T > (text : T) -> Option < Self > where T : AsRef < str > , { let text = text . as_ref () ; let len = text . len () ; if len <= INLINE_CAP { let mut buf = [0 ; INLINE_CAP] ; buf [.. len] . copy_from_slice (text . as_bytes ()) ; return Some (Repr :: Inline { len : unsafe { InlineSize :: transmute_from_u8 (len as u8) } , buf , }) ; } if len <= N_NEWLINES + N_SPACES { let bytes = text . as_bytes () ; let possible_newline_count = cmp :: min (len , N_NEWLINES) ; let newlines = bytes [.. possible_newline_count] . iter () . take_while (| & & b | b == b'\n') . count () ; let possible_space_count = len - newlines ; if possible_space_count <= N_SPACES && bytes [newlines ..] . iter () . all (| & b | b == b' ') { let spaces = possible_space_count ; let substring = & WS [N_NEWLINES - newlines .. N_NEWLINES + spaces] ; return Some (Repr :: Static (substring)) ; } } None } fn new (text : & str) -> Self { Self :: new_on_stack (text) . unwrap_or_else (| | Repr :: Heap (Arc :: from (text))) } # [inline (always)] fn len (& self) -> usize { match self { Repr :: Heap (data) => data . len () , Repr :: Static (data) => data . len () , Repr :: Inline { len , .. } => * len as usize , } } # [inline (always)] fn is_empty (& self) -> bool { match self { Repr :: Heap (data) => data . is_empty () , Repr :: Static (data) => data . is_empty () , & Repr :: Inline { len , .. } => len as u8 == 0 , } } # [inline] fn as_str (& self) -> & str { match self { Repr :: Heap (data) => data , Repr :: Static (data) => data , Repr :: Inline { len , buf } => { let len = * len as usize ; let buf = unsafe { buf . get_unchecked (.. len) } ; unsafe { :: core :: str :: from_utf8_unchecked (buf) } } } } fn ptr_eq (& self , other : & Self) -> bool { match (self , other) { (Self :: Heap (l0) , Self :: Heap (r0)) => Arc :: ptr_eq (l0 , r0) , (Self :: Static (l0) , Self :: Static (r0)) => core :: ptr :: eq (l0 , r0) , (Self :: Inline { len : l_len , buf : l_buf } , Self :: Inline { len : r_len , buf : r_buf }) => { l_len == r_len && l_buf == r_buf } _ => false , } } }
    };
}

impl_51!();