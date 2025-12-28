macro_rules! macro_138 {
    () => {
        cfg_net_unix ! { mod async_fd ; pub mod unix { # ! [doc = " Asynchronous IO structures specific to Unix-like operating systems."] pub use super :: async_fd :: { AsyncFd , AsyncFdTryNewError , AsyncFdReadyGuard , AsyncFdReadyMutGuard , TryIoError } ; } }
    };
}

macro_138!();