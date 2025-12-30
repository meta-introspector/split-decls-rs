// Generated macro for impl_34 (impl)
macro_rules! Depcrate_agentimpl_34 {
() => {
// Module: crate::agent
// Provides: {"impl_34"}
// Dependencies: {}
impl Drop for Agent { fn drop (& mut self) { unsafe { raw :: libssh2_agent_free (self . raw) } } }
};
}
