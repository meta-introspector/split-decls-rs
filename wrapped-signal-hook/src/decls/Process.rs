macro_rules! Process {
    () => {
        # [doc = " Information about process, as presented in the signal metadata."] # [derive (Copy , Clone , Debug , Eq , PartialEq)] # [non_exhaustive] pub struct Process { # [doc = " The process ID."] pub pid : pid_t , # [doc = " The user owning the process."] pub uid : uid_t , }
    };
}

Process!();