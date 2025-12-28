macro_rules! Env {
    () => {
        # [doc = " A utility to set and unset environment variables, while restoring or removing them on drop."] # [derive (Default)] pub struct Env < 'a > { altered_vars : Vec < (& 'a str , Option < OsString >) > , }
    };
}

Env!();