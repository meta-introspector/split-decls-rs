// Generated macro for ToWorker (enum)
macro_rules! Depcrate_actor_messagesToWorker {
() => {
// Module: crate::actor::messages
// Provides: {"ToWorker"}
// Dependencies: {}
# [doc = " Serializable messages to worker"] # [derive (Serialize , Deserialize , Debug)] pub (crate) enum ToWorker < W > where W : Worker , { # [doc = " Client is connected"] Connected (HandlerId) , # [doc = " Incoming message to Worker"] ProcessInput (HandlerId , W :: Input) , # [doc = " Client is disconnected"] Disconnected (HandlerId) , # [doc = " Worker should be terminated"] Destroy , }
};
}
