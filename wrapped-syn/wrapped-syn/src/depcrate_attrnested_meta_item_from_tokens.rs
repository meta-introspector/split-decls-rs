// Generated macro for nested_meta_item_from_tokens (function)
macro_rules! Depcrate_attrnested_meta_item_from_tokens {
() => {
// Module: crate::attr
// Provides: {"nested_meta_item_from_tokens"}
// Dependencies: {}
fn nested_meta_item_from_tokens (tts : & [proc_macro2 :: TokenTree]) -> Option < (NestedMetaItem , & [proc_macro2 :: TokenTree]) > { assert ! (! tts . is_empty ()) ; match tts [0] . kind { TokenNode :: Literal (ref lit) => { let lit = Lit { value : LitKind :: Other (lit . clone ()) , span : Span (tts [0] . span) , } ; Some ((NestedMetaItem :: Literal (lit) , & tts [1 ..])) } TokenNode :: Term (sym) => { let ident = Ident :: new (sym , Span (tts [0] . span)) ; if tts . len () >= 3 { if let TokenNode :: Op ('=' , Spacing :: Alone) = tts [1] . kind { if let TokenNode :: Literal (ref lit) = tts [2] . kind { let pair = MetaNameValue { ident : Ident :: new (sym , Span (tts [0] . span)) , eq_token : tokens :: Eq ([Span (tts [1] . span)]) , lit : Lit { value : LitKind :: Other (lit . clone ()) , span : Span (tts [2] . span) , } , } ; return Some ((MetaItem :: NameValue (pair) . into () , & tts [3 ..])) ; } } } if tts . len () >= 2 { if let TokenNode :: Group (Delimiter :: Parenthesis , ref inner_tts) = tts [1] . kind { let inner_tts = inner_tts . clone () . into_iter () . collect :: < Vec < _ > > () ; return match list_of_nested_meta_items_from_tokens (& inner_tts) { Some (nested_meta_items) => { let list = MetaItemList { ident : ident , paren_token : tokens :: Paren (Span (tts [1] . span)) , nested : nested_meta_items , } ; Some ((MetaItem :: List (list) . into () , & tts [2 ..])) } None => None } ; } } Some ((MetaItem :: Term (ident) . into () , & tts [1 ..])) } _ => None } }
};
}
