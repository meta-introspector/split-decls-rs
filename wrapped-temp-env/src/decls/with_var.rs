macro_rules! with_var {
    () => {
        # [doc = " Sets a single environment variable for the duration of the closure."] # [doc = ""] # [doc = " The previous value is restored when the closure completes or panics, before unwinding the"] # [doc = " panic."] # [doc = ""] # [doc = " If `value` is set to `None`, then the environment variable is unset."] pub fn with_var < K , V , F , R > (key : K , value : Option < V > , closure : F) -> R where K : AsRef < OsStr > + Clone + Eq + Hash , V : AsRef < OsStr > + Clone , F : FnOnce () -> R , { with_vars ([(key , value)] , closure) }
    };
}

with_var!();