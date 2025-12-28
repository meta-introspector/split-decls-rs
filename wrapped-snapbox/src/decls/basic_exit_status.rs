macro_rules! basic_exit_status {
    () => {
        fn basic_exit_status (status : std :: process :: ExitStatus) -> String { if let Some (code) = status . code () { code . to_string () } else { "interrupted" . to_owned () } }
    };
}

basic_exit_status!();