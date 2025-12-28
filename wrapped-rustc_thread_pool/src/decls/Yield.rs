macro_rules! Yield {
    () => {
        # [doc = " Result of [`yield_now()`] or [`yield_local()`]."] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Yield { # [doc = " Work was found and executed."] Executed , # [doc = " No available work was found."] Idle , }
    };
}

Yield!();