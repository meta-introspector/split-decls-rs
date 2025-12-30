// Generated macro for TlsListIter (struct)
macro_rules! Depcrate_msgs_codecTlsListIter {
() => {
// Module: crate::msgs::codec
// Provides: {"TlsListIter"}
// Dependencies: {}
# [doc = " An iterator over a vector of `TlsListElements`."] # [doc = ""] # [doc = " All uses _MUST_ exhaust the iterator, as errors may be delayed"] # [doc = " until the last element."] pub (crate) struct TlsListIter < 'a , T : Codec < 'a > + TlsListElement + Debug > { sub : Reader < 'a > , _t : PhantomData < T > , }
};
}
