// Generated macro for impl_228 (impl)
macro_rules! Depcrate_search_graphimpl_228 {
() => {
// Module: crate::search_graph
// Provides: {"impl_228"}
// Dependencies: {}
impl HeadUsages { fn add_usage (& mut self , path : PathKind) { match path { PathKind :: Inductive => self . inductive += 1 , PathKind :: Unknown => self . unknown += 1 , PathKind :: Coinductive => self . coinductive += 1 , PathKind :: ForcedAmbiguity => self . forced_ambiguity += 1 , } } # [doc = " This adds the usages which occurred while computing a nested goal."] # [doc = ""] # [doc = " We don't actually care about how frequently the nested goal relied"] # [doc = " on its cycle heads, only whether it did."] fn add_usages_from_nested (& mut self , usages : HeadUsages) { let HeadUsages { inductive , unknown , coinductive , forced_ambiguity } = usages ; self . inductive += if inductive == 0 { 0 } else { 1 } ; self . unknown += if unknown == 0 { 0 } else { 1 } ; self . coinductive += if coinductive == 0 { 0 } else { 1 } ; self . forced_ambiguity += if forced_ambiguity == 0 { 0 } else { 1 } ; } fn ignore_usages (& mut self , usages : HeadUsages) { let HeadUsages { inductive , unknown , coinductive , forced_ambiguity } = usages ; self . inductive = self . inductive . checked_sub (inductive) . unwrap () ; self . unknown = self . unknown . checked_sub (unknown) . unwrap () ; self . coinductive = self . coinductive . checked_sub (coinductive) . unwrap () ; self . forced_ambiguity = self . forced_ambiguity . checked_sub (forced_ambiguity) . unwrap () ; } fn is_empty (self) -> bool { let HeadUsages { inductive , unknown , coinductive , forced_ambiguity } = self ; inductive == 0 && unknown == 0 && coinductive == 0 && forced_ambiguity == 0 } }
};
}
