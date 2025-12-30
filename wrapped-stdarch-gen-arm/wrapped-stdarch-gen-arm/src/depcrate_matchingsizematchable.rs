// Generated macro for SizeMatchable (enum)
macro_rules! Depcrate_matchingSizeMatchable {
() => {
// Module: crate::matching
// Provides: {"SizeMatchable"}
// Dependencies: {}
# [derive (Debug , Clone , Serialize , Deserialize)] # [serde (untagged , deny_unknown_fields)] pub enum SizeMatchable < T > { Matched (T) , Unmatched { match_size : Option < TypeKind > , # [serde (flatten)] values : MatchSizeValues < Box < T > > , } , }
};
}
