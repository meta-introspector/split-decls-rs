macro_rules! Tiocgptpeer {
    () => {
        # [cfg (target_os = "linux")] struct Tiocgptpeer (OpenptFlags) ;
    };
}

Tiocgptpeer!();