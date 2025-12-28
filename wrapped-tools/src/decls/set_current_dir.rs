macro_rules! deps {
    () => {
        Result!();
        AutoRevertToPreviousCWD!();
    };
}

macro_rules! set_current_dir {
    () => {
        deps!();
        # [doc = " Set the current working dir to `new_cwd` and return a type that returns to the previous working dir on drop."] pub fn set_current_dir (new_cwd : impl AsRef < Path >) -> std :: io :: Result < AutoRevertToPreviousCWD > { let cwd = env :: current_dir () ? ; env :: set_current_dir (new_cwd) ? ; Ok (AutoRevertToPreviousCWD (cwd)) }
    };
}

set_current_dir!()