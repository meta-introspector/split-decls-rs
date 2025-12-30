// Generated macro for Connection (struct)
macro_rules! Depcrate_secure_transportConnection {
() => {
// Module: crate::secure_transport
// Provides: {"Connection"}
// Dependencies: {}
struct Connection < S > { stream : S , err : Option < io :: Error > , panic : Option < Box < dyn Any + Send > > , }
};
}
