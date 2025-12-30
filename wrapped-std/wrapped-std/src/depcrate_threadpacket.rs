// Generated macro for Packet (struct)
macro_rules! Depcrate_threadPacket {
() => {
// Module: crate::thread
// Provides: {"Packet"}
// Dependencies: {}
struct Packet < 'scope , T > { scope : Option < Arc < scoped :: ScopeData > > , result : UnsafeCell < Option < Result < T > > > , _marker : PhantomData < Option < & 'scope scoped :: ScopeData > > , }
};
}
