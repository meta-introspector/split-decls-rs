// Generated macro for RunCommand (trait)
macro_rules! Depcrate_mock_commandRunCommand {
() => {
// Module: crate::mock_command
// Provides: {"RunCommand"}
// Dependencies: {}
# [doc = " A trait that provides a subset of the methods of `std::process::Command`."] # [async_trait] pub trait RunCommand : fmt :: Debug + Send { # [doc = " The type returned by `spawn`."] type C : CommandChild + Send + 'static ; # [doc = " Append `arg` to the process commandline."] fn arg < S : AsRef < OsStr > > (& mut self , arg : S) -> & mut Self ; # [doc = " Append `args` to the process commandline."] fn args < S : AsRef < OsStr > > (& mut self , args : & [S]) -> & mut Self ; # [doc = " Insert or update an environment variable mapping."] fn env < K , V > (& mut self , key : K , val : V) -> & mut Self where K : AsRef < OsStr > , V : AsRef < OsStr > ; # [doc = " Add or update multiple environment variable mappings."] fn envs < I , K , V > (& mut self , vars : I) -> & mut Self where I : IntoIterator < Item = (K , V) > , K : AsRef < OsStr > , V : AsRef < OsStr > ; # [doc = " Clears the entire environment map for the child process."] fn env_clear (& mut self) -> & mut Self ; # [doc = " Set the working directory of the process to `dir`."] fn current_dir < P : AsRef < Path > > (& mut self , dir : P) -> & mut Self ; # [doc = " Set the process' stdin from `cfg`."] fn stdin (& mut self , cfg : Stdio) -> & mut Self ; # [doc = " Set the process' stdout from `cfg`."] fn stdout (& mut self , cfg : Stdio) -> & mut Self ; # [doc = " Set the process' stderr from `cfg`."] fn stderr (& mut self , cfg : Stdio) -> & mut Self ; # [doc = " Execute the process and return a process object."] async fn spawn (& mut self) -> Result < Self :: C > ; }
};
}
