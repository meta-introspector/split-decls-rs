// Generated macro for CommandCreator (trait)
macro_rules! Depcrate_mock_commandCommandCreator {
() => {
// Module: crate::mock_command
// Provides: {"CommandCreator"}
// Dependencies: {}
# [doc = " A trait that provides a means to create objects implementing `RunCommand`."] # [doc = ""] # [doc = " This is provided so that `MockCommandCreator` can have state for testing."] # [doc = " For the non-testing scenario, `ProcessCommandCreator` is simply a unit"] # [doc = " struct with a trivial implementation of this."] pub trait CommandCreator { # [doc = " The type returned by `new_command`."] type Cmd : RunCommand ; # [doc = " Create a new instance of this type."] fn new (client : & Client) -> Self ; # [doc = " Create a new object that implements `RunCommand` that can be used"] # [doc = " to create a new process."] fn new_command < S : AsRef < OsStr > > (& mut self , program : S) -> Self :: Cmd ; }
};
}
