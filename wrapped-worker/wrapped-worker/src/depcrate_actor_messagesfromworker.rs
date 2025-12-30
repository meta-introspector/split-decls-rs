// Generated macro for FromWorker (enum)
macro_rules! Depcrate_actor_messagesFromWorker {
() => {
// Module: crate::actor::messages
// Provides: {"FromWorker"}
// Dependencies: {}
# [doc = " Serializable messages sent by worker to consumer"] # [derive (Serialize , Deserialize , Debug)] pub (crate) enum FromWorker < W > where W : Worker , { # [doc = " Worker sends this message when `wasm` bundle has loaded."] WorkerLoaded , # [doc = " Outgoing message to consumer"] ProcessOutput (HandlerId , W :: Output) , }
};
}
