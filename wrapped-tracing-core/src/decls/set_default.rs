macro_rules! deps {
    () => {
        DefaultGuard!();
        Dispatch!();
        State!();
    };
}

macro_rules! set_default {
    () => {
        deps!();
        # [doc = " Sets the dispatch as the default dispatch for the duration of the lifetime"] # [doc = " of the returned DefaultGuard"] # [doc = ""] # [doc = " <pre class=\"ignore\" style=\"white-space:normal;font:inherit;\">"] # [doc = "     <strong>Note</strong>: This function required the Rust standard library."] # [doc = "     <code>no_std</code> users should use <a href=\"fn.set_global_default.html\">"] # [doc = "     <code>set_global_default</code></a> instead."] # [doc = " </pre>"] # [doc = ""] # [doc = " [`set_global_default`]: set_global_default"] # [cfg (feature = "std")] # [cfg_attr (docsrs , doc (cfg (feature = "std")))] # [must_use = "Dropping the guard unregisters the dispatcher."] pub fn set_default (dispatcher : & Dispatch) -> DefaultGuard { State :: set_default (dispatcher . clone ()) }
    };
}

set_default!();