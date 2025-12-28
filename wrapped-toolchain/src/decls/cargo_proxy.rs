macro_rules! cargo_proxy {
    () => {
        # [doc = " Looks up the binary in the cargo home directory if it exists."] fn cargo_proxy (executable_name : & str) -> Option < Utf8PathBuf > { let mut path = get_cargo_home () ? ; path . push ("bin") ; path . push (executable_name) ; probe_for_binary (path) }
    };
}

cargo_proxy!();