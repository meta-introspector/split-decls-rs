// Generated macro for ReactorOutput (enum)
macro_rules! Depcrate_reactor_messagesReactorOutput {
() => {
// Module: crate::reactor::messages
// Provides: {"ReactorOutput"}
// Dependencies: {}
# [doc = " The Bridge Output."] # [derive (Debug , Serialize , Deserialize)] pub enum ReactorOutput < O > { # [doc = " An output message has been received."] Output (O) , # [doc = " Reactor for current bridge has exited."] Finish , }
};
}
