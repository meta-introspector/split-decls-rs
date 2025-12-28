macro_rules! AutoRevertToPreviousCWD {
    () => {
        # [doc = " A utility to set the current working dir to the given value, on drop."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Note that this will panic if the CWD cannot be set on drop."] # [derive (Debug)] # [must_use] pub struct AutoRevertToPreviousCWD (PathBuf) ;
    };
}

AutoRevertToPreviousCWD!();