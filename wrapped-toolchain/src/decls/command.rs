macro_rules! command {
    () => {
        # [allow (clippy :: disallowed_types)] pub fn command < H > (cmd : impl AsRef < OsStr > , working_directory : impl AsRef < Path > , extra_env : & std :: collections :: HashMap < String , Option < String > , H > ,) -> Command { # [allow (clippy :: disallowed_methods)] let mut cmd = Command :: new (cmd) ; cmd . current_dir (working_directory) ; cmd . env (NO_RUSTUP_AUTO_INSTALL_ENV . 0 , NO_RUSTUP_AUTO_INSTALL_ENV . 1) ; for env in extra_env { match env { (key , Some (val)) => cmd . env (key , val) , (key , None) => cmd . env_remove (key) , } ; } cmd }
    };
}

command!()