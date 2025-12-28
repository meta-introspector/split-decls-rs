macro_rules! deps {
    () => {
        Comparator!();
        Version!();
    };
}

macro_rules! matches_less {
    () => {
        deps!();
        fn matches_less (cmp : & Comparator , ver : & Version) -> bool { if ver . major != cmp . major { return ver . major < cmp . major ; } match cmp . minor { None => return false , Some (minor) => { if ver . minor != minor { return ver . minor < minor ; } } } match cmp . patch { None => return false , Some (patch) => { if ver . patch != patch { return ver . patch < patch ; } } } ver . pre < cmp . pre }
    };
}

matches_less!()