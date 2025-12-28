macro_rules! toml {
    () => {
        pub (crate) fn toml () -> toml :: Value { let mut rustflags = vec ! ["--cfg" , "trybuild" , "--verbose"] ; for & lint in IGNORED_LINTS { rustflags . push ("-A") ; rustflags . push (lint) ; } if let Some (flags) = env :: var_os ("RUSTFLAGS") { if flags . to_string_lossy () . contains ("-C instrument-coverage") { rustflags . extend (["-C" , "instrument-coverage"]) ; } } toml :: Value :: try_from (rustflags) . unwrap () }
    };
}

toml!();