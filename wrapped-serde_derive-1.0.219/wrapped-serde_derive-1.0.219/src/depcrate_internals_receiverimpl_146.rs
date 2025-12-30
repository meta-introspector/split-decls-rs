// Generated macro for impl_146 (impl)
macro_rules! Depcrate_internals_receiverimpl_146 {
() => {
// Module: crate::internals::receiver
// Provides: {"impl_146"}
// Dependencies: {}
impl ReplaceReceiver < '_ > { fn self_ty (& self , span : Span) -> TypePath { let tokens = self . 0 . to_token_stream () ; let respanned = respan (tokens , span) ; syn :: parse2 (respanned) . unwrap () } fn self_to_qself (& self , qself : & mut Option < QSelf > , path : & mut Path) { if path . leading_colon . is_some () || path . segments [0] . ident != "Self" { return ; } if path . segments . len () == 1 { self . self_to_expr_path (path) ; return ; } let span = path . segments [0] . ident . span () ; * qself = Some (QSelf { lt_token : Token ! [<] (span) , ty : Box :: new (Type :: Path (self . self_ty (span))) , position : 0 , as_token : None , gt_token : Token ! [>] (span) , }) ; path . leading_colon = Some (* * path . segments . pairs () . next () . unwrap () . punct () . unwrap ()) ; let segments = mem :: take (& mut path . segments) ; path . segments = segments . into_pairs () . skip (1) . collect () ; } fn self_to_expr_path (& self , path : & mut Path) { let self_ty = self . self_ty (path . segments [0] . ident . span ()) ; let variant = mem :: replace (path , self_ty . path) ; for segment in & mut path . segments { if let PathArguments :: AngleBracketed (bracketed) = & mut segment . arguments { if bracketed . colon2_token . is_none () && ! bracketed . args . is_empty () { bracketed . colon2_token = Some (< Token ! [::] > :: default ()) ; } } } if variant . segments . len () > 1 { path . segments . push_punct (< Token ! [::] > :: default ()) ; path . segments . extend (variant . segments . into_pairs () . skip (1)) ; } } }
};
}
