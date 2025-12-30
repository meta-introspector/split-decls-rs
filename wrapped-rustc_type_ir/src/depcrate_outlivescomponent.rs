// Generated macro for Component (enum)
macro_rules! Depcrate_outlivesComponent {
() => {
// Module: crate::outlives
// Provides: {"Component"}
// Dependencies: {}
# [derive_where (Debug ; I : Interner)] pub enum Component < I : Interner > { Region (I :: Region) , Param (I :: ParamTy) , Placeholder (I :: PlaceholderTy) , UnresolvedInferenceVariable (ty :: InferTy) , Alias (ty :: AliasTy < I >) , EscapingAlias (Vec < Component < I > >) , }
};
}
