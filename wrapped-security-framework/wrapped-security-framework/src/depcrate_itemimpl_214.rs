// Generated macro for impl_214 (impl)
macro_rules! Depcrate_itemimpl_214 {
() => {
// Module: crate::item
// Provides: {"impl_214"}
// Dependencies: {}
impl From < Option < bool > > for CloudSync { # [inline] fn from (is_sync : Option < bool >) -> Self { match is_sync { Some (true) => Self :: MatchSyncYes , Some (false) => Self :: MatchSyncNo , None => Self :: MatchSyncAny , } } }
};
}
