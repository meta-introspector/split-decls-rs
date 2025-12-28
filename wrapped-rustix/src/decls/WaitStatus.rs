macro_rules! WaitStatus {
    () => {
        # [doc = " The status of a child process after calling [`wait`]/[`waitpid`]."] # [derive (Clone , Copy)] # [repr (transparent)] pub struct WaitStatus (i32) ;
    };
}

WaitStatus!();