// Generated macro for impl_45 (impl)
macro_rules! Depcrateimpl_45 {
() => {
// Module: crate
// Provides: {"impl_45"}
// Dependencies: {}
impl ResumptionParam { fn from_subcommand (cmd : & Command) -> Self { match cmd { Command :: Handshake { .. } => Self :: No , Command :: HandshakeResume { .. } => Self :: SessionId , Command :: HandshakeTicket { .. } => Self :: Tickets , _ => todo ! ("unhandled subcommand {cmd:?}") , } } fn as_handshake_kind (& self) -> HandshakeKind { match * self { Self :: No => HandshakeKind :: Full , Self :: SessionId | Self :: Tickets => HandshakeKind :: Resumed , } } fn label (& self) -> & 'static str { match * self { Self :: No => "no-resume" , Self :: SessionId => "sessionid" , Self :: Tickets => "tickets" , } } }
};
}
