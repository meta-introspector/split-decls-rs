macro_rules! deps {
    () => {
        TokenConverter!();
        StaticRawConverter!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl < S > TokenConverter < S > for StaticRawConverter < '_ , S > where S : Copy , { type Token = usize ; fn convert_doc_comment (& self , & token : & usize , span : S , builder : & mut tt :: TopSubtreeBuilder < S >) { let text = self . lexed . text (token) ; convert_doc_comment (& doc_comment (text) , span , self . mode , builder) ; } fn bump (& mut self) -> Option < (Self :: Token , TextRange) > { if self . pos == self . lexed . len () { return None ; } let token = self . pos ; self . pos += 1 ; let range = self . lexed . text_range (token) ; let range = TextRange :: new (range . start . try_into () . ok () ? , range . end . try_into () . ok () ?) ; Some ((token , range)) } fn peek (& self) -> Option < Self :: Token > { if self . pos == self . lexed . len () { return None ; } Some (self . pos) } fn span_for (& self , _ : TextRange) -> S { self . span } fn call_site (& self) -> S { self . span } }
    };
}

impl_24!()