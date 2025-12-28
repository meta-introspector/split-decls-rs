macro_rules! to_disambiguator {
    () => {
        # [doc = " Converts a number to a disambiguator (see"] # [doc = " <https://rust-lang.github.io/rfcs/2603-rust-symbol-name-mangling-v0.html>)."] fn to_disambiguator (num : u64) -> String { if let Some (num) = num . checked_sub (1) { format ! ("s{}_" , num . to_base (ALPHANUMERIC_ONLY)) } else { "s_" . to_string () } }
    };
}

to_disambiguator!()