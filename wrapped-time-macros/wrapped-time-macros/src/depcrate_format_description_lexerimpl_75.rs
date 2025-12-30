// Generated macro for impl_75 (impl)
macro_rules! Depcrate_format_description_lexerimpl_75 {
() => {
// Module: crate::format_description::lexer
// Provides: {"impl_75"}
// Dependencies: {}
impl < 'iter , 'token : 'iter , I : Iterator < Item = Result < Token < 'token > , Error > > + 'iter > Lexed < I > { pub (super) fn peek (& mut self) -> Option < & I :: Item > { self . iter . peek () } pub (super) fn next_if_whitespace (& mut self) -> Option < Spanned < & 'token [u8] > > { if let Some (& Ok (Token :: ComponentPart { kind : ComponentKind :: Whitespace , value , })) = self . peek () { self . next () ; Some (value) } else { None } } pub (super) fn next_if_not_whitespace (& mut self) -> Option < Spanned < & 'token [u8] > > { if let Some (& Ok (Token :: ComponentPart { kind : ComponentKind :: NotWhitespace , value , })) = self . peek () { self . next () ; Some (value) } else { None } } pub (super) fn next_if_opening_bracket (& mut self) -> Option < Location > { if let Some (& Ok (Token :: Bracket { kind : BracketKind :: Opening , location , })) = self . peek () { self . next () ; Some (location) } else { None } } pub (super) fn peek_closing_bracket (& 'iter mut self) -> Option < & 'iter Location > { if let Some (Ok (Token :: Bracket { kind : BracketKind :: Closing , location , })) = self . peek () { Some (location) } else { None } } pub (super) fn next_if_closing_bracket (& mut self) -> Option < Location > { if let Some (& Ok (Token :: Bracket { kind : BracketKind :: Closing , location , })) = self . peek () { self . next () ; Some (location) } else { None } } }
};
}
