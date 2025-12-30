// Generated macro for TokenConverter (trait)
macro_rules! DepcrateTokenConverter {
() => {
// Module: crate
// Provides: {"TokenConverter"}
// Dependencies: {}
trait TokenConverter < S > : Sized { type Token : SrcToken < Self , S > ; fn convert_doc_comment (& self , token : & Self :: Token , span : S , builder : & mut tt :: TopSubtreeBuilder < S > ,) ; fn bump (& mut self) -> Option < (Self :: Token , TextRange) > ; fn peek (& self) -> Option < Self :: Token > ; fn span_for (& self , range : TextRange) -> S ; fn call_site (& self) -> S ; }
};
}
