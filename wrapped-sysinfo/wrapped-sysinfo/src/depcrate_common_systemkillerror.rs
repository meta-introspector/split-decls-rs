// Generated macro for KillError (enum)
macro_rules! Depcrate_common_systemKillError {
() => {
// Module: crate::common::system
// Provides: {"KillError"}
// Dependencies: {}
# [doc = " Enum describing possible [`Process::kill_and_wait`] errors."] # [derive (Clone , Copy , Debug , PartialEq , Eq)] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize))] pub enum KillError { # [doc = " This signal doesn't exist on this platform."] SignalDoesNotExist , # [doc = " The signal failed to be sent to the target process."] FailedToSendSignal , }
};
}
