macro_rules! NO_RUSTUP_AUTO_INSTALL_ENV {
    () => {
        pub const NO_RUSTUP_AUTO_INSTALL_ENV : (& str , & str) = ("RUSTUP_AUTO_INSTALL" , "0") ;
    };
}

NO_RUSTUP_AUTO_INSTALL_ENV!();