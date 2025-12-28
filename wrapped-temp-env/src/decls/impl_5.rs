macro_rules! deps {
    () => {
        RestoreEnv!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl Drop for RestoreEnv < '_ > { fn drop (& mut self) { for (var , value) in self . env . iter () { update_env (var , value . as_ref () . map (| v | v . as_os_str ())) ; } } }
    };
}

impl_5!();