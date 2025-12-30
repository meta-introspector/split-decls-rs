// Generated macro for macro_9697 (macro)
macro_rules! Depcrate_shared_rpcdcemacro_9697 {
() => {
// Module: crate::shared::rpcdce
// Provides: {"macro_9697"}
// Dependencies: {}
STRUCT ! { struct RPC_INTERFACE_TEMPLATEA { Version : c_ulong , IfSpec : RPC_IF_HANDLE , MgrTypeUuid : * mut UUID , MgrEpv : * mut RPC_MGR_EPV , Flags : c_uint , MaxCalls : c_uint , MaxRpcSize : c_uint , IfCallback : * mut RPC_IF_CALLBACK_FN , UuidVector : * mut UUID_VECTOR , Annotation : RPC_CSTR , SecurityDescriptor : * mut c_void , } }
};
}
