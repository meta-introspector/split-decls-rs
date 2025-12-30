// Generated macro for impl_109 (impl)
macro_rules! Depcrate_hygieneimpl_109 {
() => {
// Module: crate::hygiene
// Provides: {"impl_109"}
// Dependencies: {}
impl Span { # [doc = " Reuses the span but adds information like the kind of the desugaring and features that are"] # [doc = " allowed inside this span."] pub fn mark_with_reason (self , allow_internal_unstable : Option < Arc < [Symbol] > > , reason : DesugaringKind , edition : Edition , ctx : impl HashStableContext ,) -> Span { let expn_data = ExpnData { allow_internal_unstable , .. ExpnData :: default (ExpnKind :: Desugaring (reason) , self , edition , None , None) } ; let expn_id = LocalExpnId :: fresh (expn_data , ctx) ; self . apply_mark (expn_id . to_expn_id () , Transparency :: Transparent) } }
};
}
