// Generated macro for Agent (struct)
macro_rules! Depcrate_agentAgent {
() => {
// Module: crate::agent
// Provides: {"Agent"}
// Dependencies: {}
# [doc = " A structure representing a connection to an SSH agent."] # [doc = ""] # [doc = " Agents can be used to authenticate a session."] pub struct Agent { raw : * mut raw :: LIBSSH2_AGENT , sess : Arc < Mutex < SessionInner > > , }
};
}
