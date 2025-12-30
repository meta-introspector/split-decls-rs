// Generated macro for impl_212 (impl)
macro_rules! Depcrate_matchingimpl_212 {
() => {
// Module: crate::matching
// Provides: {"impl_212"}
// Dependencies: {}
impl < T : Clone > SizeMatchable < T > { pub fn perform_match (& mut self , ctx : & LocalContext) -> context :: Result { match self { Self :: Unmatched { match_size : None , values : MatchSizeValues { default , .. } , } => * self = Self :: Matched (* default . to_owned ()) , Self :: Unmatched { match_size : Some (ty) , values , } => * self = Self :: Matched (* values . get (ty , ctx) ? . to_owned ()) , _ => { } } Ok (()) } }
};
}
