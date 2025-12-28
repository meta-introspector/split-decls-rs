macro_rules! Action {
    () => {
        # [doc = " Test action, see [`Assert`][crate::Assert]"] # [derive (Copy , Clone , Debug , PartialEq , Eq , Default)] pub enum Action { # [doc = " Do not run the test"] Skip , # [doc = " Ignore test failures"] Ignore , # [doc = " Fail on mismatch"] # [default] Verify , # [doc = " Overwrite on mismatch"] Overwrite , }
    };
}

Action!()