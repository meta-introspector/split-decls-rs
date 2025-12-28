macro_rules! deps {
    () => {
        Comparator!();
        Version!();
    };
}

macro_rules! matches_exact {
    () => {
        deps!();
        fn matches_exact (cmp : & Comparator , ver : & Version) -> bool { if ver . major != cmp . major { return false ; } if let Some (minor) = cmp . minor { if ver . minor != minor { return false ; } } if let Some (patch) = cmp . patch { if ver . patch != patch { return false ; } } ver . pre == cmp . pre }
    };
}

matches_exact!();