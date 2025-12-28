macro_rules! deps {
    () => {
        SynToken!();
        SpanMapper!();
        TokenConverter!();
        Converter!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        impl < S , SpanMap > TokenConverter < S > for Converter < SpanMap , S > where S : Copy , SpanMap : SpanMapper < S > , { type Token = SynToken < S > ; fn convert_doc_comment (& self , token : & Self :: Token , span : S , builder : & mut tt :: TopSubtreeBuilder < S > ,) { convert_doc_comment (token . token () , span , self . mode , builder) ; } fn bump (& mut self) -> Option < (Self :: Token , TextRange) > { if let Some ((punct , offset)) = self . punct_offset . clone () && usize :: from (offset) + 1 < punct . text () . len () { let offset = offset + TextSize :: of ('.') ; let range = punct . text_range () ; self . punct_offset = Some ((punct . clone () , offset)) ; let range = TextRange :: at (range . start () + offset , TextSize :: of ('.')) ; return Some ((SynToken :: Punct { token : punct , offset : u32 :: from (offset) as usize } , range ,)) ; } if let Some (leaf) = self . current_leaves . pop () { if self . current_leaves . is_empty () { self . current = self . next_token () ; } return Some ((SynToken :: Leaf (leaf) , TextRange :: empty (TextSize :: new (0)))) ; } let curr = self . current . clone () ? ; if ! self . range . contains_range (curr . text_range ()) { return None ; } self . current = self . next_token () ; let token = if curr . kind () . is_punct () { self . punct_offset = Some ((curr . clone () , 0 . into ())) ; let range = curr . text_range () ; let range = TextRange :: at (range . start () , TextSize :: of ('.')) ; (SynToken :: Punct { token : curr , offset : 0_usize } , range) } else { self . punct_offset = None ; let range = curr . text_range () ; (SynToken :: Ordinary (curr) , range) } ; Some (token) } fn peek (& self) -> Option < Self :: Token > { if let Some ((punct , mut offset)) = self . punct_offset . clone () { offset += TextSize :: of ('.') ; if usize :: from (offset) < punct . text () . len () { return Some (SynToken :: Punct { token : punct , offset : usize :: from (offset) }) ; } } let curr = self . current . clone () ? ; if ! self . range . contains_range (curr . text_range ()) { return None ; } let token = if curr . kind () . is_punct () { SynToken :: Punct { token : curr , offset : 0_usize } } else { SynToken :: Ordinary (curr) } ; Some (token) } fn span_for (& self , range : TextRange) -> S { self . map . span_for (range) } fn call_site (& self) -> S { self . call_site } }
    };
}

impl_30!()