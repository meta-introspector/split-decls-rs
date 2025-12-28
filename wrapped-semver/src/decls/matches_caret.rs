macro_rules! deps {
    () => {
        Comparator!();
        Version!();
    };
}

macro_rules! matches_caret {
    () => {
        deps!();
        fn matches_caret (cmp : & Comparator , ver : & Version) -> bool { if ver . major != cmp . major { return false ; } let minor = match cmp . minor { None => return true , Some (minor) => minor , } ; let patch = match cmp . patch { None => { if cmp . major > 0 { return ver . minor >= minor ; } else { return ver . minor == minor ; } } Some (patch) => patch , } ; if cmp . major > 0 { if ver . minor != minor { return ver . minor > minor ; } else if ver . patch != patch { return ver . patch > patch ; } } else if minor > 0 { if ver . minor != minor { return false ; } else if ver . patch != patch { return ver . patch > patch ; } } else if ver . minor != minor || ver . patch != patch { return false ; } ver . pre >= cmp . pre }
    };
}

matches_caret!()