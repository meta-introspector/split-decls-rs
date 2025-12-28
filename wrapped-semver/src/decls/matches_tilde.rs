macro_rules! deps {
    () => {
        Version!();
        Comparator!();
    };
}

macro_rules! matches_tilde {
    () => {
        deps!();
        fn matches_tilde (cmp : & Comparator , ver : & Version) -> bool { if ver . major != cmp . major { return false ; } if let Some (minor) = cmp . minor { if ver . minor != minor { return false ; } } if let Some (patch) = cmp . patch { if ver . patch != patch { return ver . patch > patch ; } } ver . pre >= cmp . pre }
    };
}

matches_tilde!()