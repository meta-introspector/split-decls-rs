macro_rules! initialize_checked {
    () => {
        pub fn initialize_checked (report_warning : impl FnOnce (& 'static str)) { let client_checked = match & * GLOBAL_CLIENT { Ok (client) => client . clone () , Err (e) => { report_warning (e) ; default_client () } } ; GLOBAL_CLIENT_CHECKED . set (client_checked) . ok () ; }
    };
}

initialize_checked!();