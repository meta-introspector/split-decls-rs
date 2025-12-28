macro_rules! Chld {
    () => {
        # [doc = " A child changed its state."] # [derive (Copy , Clone , Debug , Eq , PartialEq)] # [non_exhaustive] pub enum Chld { # [doc = " The child exited normally."] Exited , # [doc = " It got killed by a signal."] Killed , # [doc = " It got killed by a signal and dumped core."] Dumped , # [doc = " The child was trapped by a `SIGTRAP` signal."] Trapped , # [doc = " The child got stopped."] Stopped , # [doc = " The child continued (after being stopped)."] Continued , }
    };
}

Chld!()