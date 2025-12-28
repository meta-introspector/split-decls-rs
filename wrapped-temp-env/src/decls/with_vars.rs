macro_rules! deps {
    () => {
        RestoreEnv!();
    };
}

macro_rules! with_vars {
    () => {
        deps!();
        # [doc = " Sets environment variables for the duration of the closure."] # [doc = ""] # [doc = " The previous values are restored when the closure completes or panics, before unwinding the"] # [doc = " panic."] # [doc = ""] # [doc = " If a `value` is set to `None`, then the environment variable is unset."] # [doc = ""] # [doc = " If the variable with the same name is set multiple times, the last one wins."] pub fn with_vars < K , V , F , R > (kvs : impl AsRef < [(K , Option < V >)] > , closure : F) -> R where K : AsRef < OsStr > + Clone + Eq + Hash , V : AsRef < OsStr > + Clone , F : FnOnce () -> R , { let old_env = RestoreEnv :: capture (SERIAL_TEST . lock () , kvs . as_ref () . iter () . map (| (k , _) | k . as_ref ()) ,) ; for (key , value) in kvs . as_ref () { update_env (key , value . as_ref ()) ; } let retval = closure () ; drop (old_env) ; retval }
    };
}

with_vars!()