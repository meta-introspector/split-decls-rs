macro_rules! deps {
    () => {
        Inline!();
    };
}

macro_rules! impl_93 {
    () => {
        deps!();
        impl Inline { pub (crate) fn trimmed (& self) -> String { let mut data = self . data ; if data . contains ('\n') { data = data . strip_prefix ('\n') . unwrap_or (data) ; data = data . strip_suffix ('\n') . unwrap_or (data) ; } data . to_owned () } }
    };
}

impl_93!()