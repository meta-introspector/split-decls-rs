macro_rules! deps {
    () => {
        LocalDefId!();
        SpanData!();
        Span!();
        SyntaxContext!();
        InlineParent!();
    };
}

macro_rules! impl_155 {
    () => {
        deps!();
        impl InlineParent { # [inline] fn data (self) -> SpanData { let len = (self . len_with_tag & ! PARENT_TAG) as u32 ; debug_assert ! (len <= MAX_LEN) ; SpanData { lo : BytePos (self . lo) , hi : BytePos (self . lo . debug_strict_add (len)) , ctxt : SyntaxContext :: root () , parent : Some (LocalDefId { local_def_index : DefIndex :: from_u16 (self . parent) }) , } } # [inline] fn span (lo : u32 , len : u16 , parent : u16) -> Span { let (lo_or_index , len_with_tag_or_marker , ctxt_or_parent_or_marker) = (lo , PARENT_TAG | len , parent) ; Span { lo_or_index , len_with_tag_or_marker , ctxt_or_parent_or_marker } } # [inline] fn from_span (span : Span) -> InlineParent { let (lo , len_with_tag , parent) = (span . lo_or_index , span . len_with_tag_or_marker , span . ctxt_or_parent_or_marker) ; InlineParent { lo , len_with_tag , parent } } }
    };
}

impl_155!();