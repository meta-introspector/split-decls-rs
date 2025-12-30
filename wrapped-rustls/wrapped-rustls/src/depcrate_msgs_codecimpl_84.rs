// Generated macro for impl_84 (impl)
macro_rules! Depcrate_msgs_codecimpl_84 {
() => {
// Module: crate::msgs::codec
// Provides: {"impl_84"}
// Dependencies: {}
impl < 'a , T : Codec < 'a > + TlsListElement + Debug > Iterator for TlsListIter < 'a , T > { type Item = Result < T , InvalidMessage > ; fn next (& mut self) -> Option < Self :: Item > { match self . sub . any_left () { true => Some (T :: read (& mut self . sub)) , false => None , } } }
};
}
