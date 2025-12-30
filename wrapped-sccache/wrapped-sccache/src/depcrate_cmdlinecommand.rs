// Generated macro for Command (enum)
macro_rules! Depcrate_cmdlineCommand {
() => {
// Module: crate::cmdline
// Provides: {"Command"}
// Dependencies: {}
# [doc = " A specific command to run."] pub enum Command { # [doc = " Show cache statistics and exit."] ShowStats (StatsFormat , bool) , # [doc = " Run background server."] InternalStartServer , # [doc = " Start background server as a subprocess."] StartServer , # [doc = " Stop background server."] StopServer , # [doc = " Zero cache statistics and exit."] ZeroStats , # [doc = " Show the status of the distributed client."] DistStatus , # [doc = " Perform a login to authenticate for distributed compilation."] DistAuth , # [doc = " Package a toolchain for distributed compilation (executable, out)"] PackageToolchain (PathBuf , PathBuf) , # [doc = " Run a compiler command."] Compile { # [doc = " The binary to execute."] exe : OsString , # [doc = " The commandline arguments to pass to `exe`."] cmdline : Vec < OsString > , # [doc = " The directory in which to execute the command."] cwd : PathBuf , # [doc = " The environment variables to use for execution."] env_vars : Vec < (OsString , OsString) > , } , DebugPreprocessorCacheEntries , }
};
}
