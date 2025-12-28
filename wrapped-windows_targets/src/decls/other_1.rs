macro_rules! other_1 {
    () => {
        pub macro link_dylib { ($ library : literal $ abi : literal $ ($ link_name : literal) ? $ (# [$ doc : meta]) ? fn $ ($ function : tt) *) => (# [link (name = "kernel32")] unsafe extern $ abi { $ (# [link_name =$ link_name]) ? pub fn $ ($ function) *; }) }
    };
}

other_1!()