// Generated macro for CloudSync (enum)
macro_rules! Depcrate_itemCloudSync {
() => {
// Module: crate::item
// Provides: {"CloudSync"}
// Dependencies: {}
# [doc = " Specifies whether a search should match cloud-synchronized items."] # [derive (Debug , Copy , Clone)] pub enum CloudSync { # [doc = " Match only items that are cloud-synchronized."] MatchSyncYes , # [doc = " Match only items that are not cloud-synchronized."] MatchSyncNo , # [doc = " Match items whether they are cloud-synchronized or not."] MatchSyncAny , }
};
}
