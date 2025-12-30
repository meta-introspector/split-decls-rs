// Generated macro for Reconnect (struct)
macro_rules! Depcrate_reconnectReconnect {
() => {
// Module: crate::reconnect
// Provides: {"Reconnect"}
// Dependencies: {}
# [doc = " Reconnect to failed services."] pub struct Reconnect < M , Target > where M : Service < Target > , { mk_service : M , state : State < M :: Future , M :: Response > , target : Target , error : Option < M :: Error > , }
};
}
