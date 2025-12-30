// Generated macro for impl_51 (impl)
macro_rules! Depcrateimpl_51 {
() => {
// Module: crate
// Provides: {"impl_51"}
// Dependencies: {}
impl < Ctx : Copy > TokenConverter < SpanData < Ctx > > for RawConverter < '_ , Ctx > where SpanData < Ctx > : Copy , { type Token = usize ; fn convert_doc_comment (& self , & token : & usize , span : SpanData < Ctx > , builder : & mut tt :: TopSubtreeBuilder < SpanData < Ctx > > ,) { let text = self . lexed . text (token) ; convert_doc_comment (& doc_comment (text) , span , self . mode , builder) ; } fn bump (& mut self) -> Option < (Self :: Token , TextRange) > { if self . pos == self . lexed . len () { return None ; } let token = self . pos ; self . pos += 1 ; let range = self . lexed . text_range (token) ; let range = TextRange :: new (range . start . try_into () . ok () ? , range . end . try_into () . ok () ?) ; Some ((token , range)) } fn peek (& self) -> Option < Self :: Token > { if self . pos == self . lexed . len () { return None ; } Some (self . pos) } fn span_for (& self , range : TextRange) -> SpanData < Ctx > { SpanData { range , anchor : self . anchor , ctx : self . ctx } } fn call_site (& self) -> SpanData < Ctx > { SpanData { range : TextRange :: empty (0 . into ()) , anchor : self . anchor , ctx : self . ctx } } }
};
}
