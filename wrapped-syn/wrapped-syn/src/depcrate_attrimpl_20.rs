// Generated macro for impl_20 (impl)
macro_rules! Depcrate_attrimpl_20 {
() => {
// Module: crate::attr
// Provides: {"impl_20"}
// Dependencies: {}
impl Attribute { # [doc = " Parses the tokens after the path as a [`MetaItem`](enum.MetaItem.html) if possible."] pub fn meta_item (& self) -> Option < MetaItem > { let name = if self . path . segments . len () == 1 { & self . path . segments . get (0) . item () . ident } else { return None ; } ; if self . tts . is_empty () { return Some (MetaItem :: Term (name . clone ())) ; } if self . tts . len () == 1 { if let TokenNode :: Group (Delimiter :: Parenthesis , ref ts) = self . tts [0] . 0 . kind { let tokens = ts . clone () . into_iter () . collect :: < Vec < _ > > () ; if let Some (nested_meta_items) = list_of_nested_meta_items_from_tokens (& tokens) { return Some (MetaItem :: List (MetaItemList { paren_token : tokens :: Paren (Span (self . tts [0] . 0 . span)) , ident : name . clone () , nested : nested_meta_items , })) ; } } } if self . tts . len () == 2 { if let TokenNode :: Op ('=' , Spacing :: Alone) = self . tts [0] . 0 . kind { if let TokenNode :: Literal (ref lit) = self . tts [1] . 0 . kind { return Some (MetaItem :: NameValue (MetaNameValue { ident : name . clone () , eq_token : tokens :: Eq ([Span (self . tts [0] . 0 . span)]) , lit : Lit { value : LitKind :: Other (lit . clone ()) , span : Span (self . tts [1] . 0 . span) , } , })) ; } } } None } }
};
}
