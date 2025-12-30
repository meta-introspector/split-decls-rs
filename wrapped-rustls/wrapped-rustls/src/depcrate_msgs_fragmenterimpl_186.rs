// Generated macro for impl_186 (impl)
macro_rules! Depcrate_msgs_fragmenterimpl_186 {
() => {
// Module: crate::msgs::fragmenter
// Provides: {"impl_186"}
// Dependencies: {}
impl < 'a > Iterator for Chunker < 'a > { type Item = OutboundChunks < 'a > ; fn next (& mut self) -> Option < Self :: Item > { if self . payload . is_empty () { return None ; } let (before , after) = self . payload . split_at (self . limit) ; self . payload = after ; Some (before) } }
};
}
