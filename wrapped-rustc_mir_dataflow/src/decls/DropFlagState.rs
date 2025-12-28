macro_rules! DropFlagState {
    () => {
        # [doc = " The value of an inserted drop flag."] # [derive (Debug , PartialEq , Eq , Copy , Clone)] pub enum DropFlagState { # [doc = " The tracked value is initialized and needs to be dropped when leaving its scope."] Present , # [doc = " The tracked value is uninitialized or was moved out of and does not need to be dropped when"] # [doc = " leaving its scope."] Absent , }
    };
}

DropFlagState!()