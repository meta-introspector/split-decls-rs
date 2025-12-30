// Generated macro for impl_217 (impl)
macro_rules! Depcrate_matchingimpl_217 {
() => {
// Module: crate::matching
// Provides: {"impl_217"}
// Dependencies: {}
impl < T : Clone > KindMatchable < T > { pub fn perform_match (& mut self , ctx : & LocalContext) -> context :: Result { match self { Self :: Unmatched { match_kind : None , values : MatchKindValues { default , .. } , } => * self = Self :: Matched (* default . to_owned ()) , Self :: Unmatched { match_kind : Some (ty) , values , } => * self = Self :: Matched (* values . get (ty , ctx) ? . to_owned ()) , _ => { } } Ok (()) } }
};
}
