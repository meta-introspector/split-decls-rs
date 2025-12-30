// Generated macro for ConnectionCore (struct)
macro_rules! Depcrate_connConnectionCore {
() => {
// Module: crate::conn
// Provides: {"ConnectionCore"}
// Dependencies: {}
pub (crate) struct ConnectionCore < Side : SideData > { pub (crate) state : Result < Box < dyn State < Side > > , Error > , pub (crate) side : Side , pub (crate) common_state : CommonState , pub (crate) hs_deframer : HandshakeDeframer , # [doc = " We limit consecutive empty fragments to avoid a route for the peer to send"] # [doc = " us significant but fruitless traffic."] seen_consecutive_empty_fragments : u8 , }
};
}
