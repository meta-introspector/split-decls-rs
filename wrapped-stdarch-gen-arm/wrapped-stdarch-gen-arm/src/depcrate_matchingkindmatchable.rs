// Generated macro for KindMatchable (enum)
macro_rules! Depcrate_matchingKindMatchable {
() => {
// Module: crate::matching
// Provides: {"KindMatchable"}
// Dependencies: {}
# [derive (Debug , Clone , Serialize , Deserialize)] # [serde (untagged , deny_unknown_fields)] pub enum KindMatchable < T > { Matched (T) , Unmatched { match_kind : Option < TypeKind > , # [serde (flatten)] values : MatchKindValues < Box < T > > , } , }
};
}
