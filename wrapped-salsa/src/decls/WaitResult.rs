macro_rules! WaitResult {
    () => {
        # [derive (Copy , Clone , Debug)] pub (super) enum WaitResult { Completed , Panicked , }
    };
}

WaitResult!()