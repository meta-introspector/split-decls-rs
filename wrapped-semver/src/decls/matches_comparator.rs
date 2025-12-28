macro_rules! deps {
    () => {
        Comparator!();
        Version!();
    };
}

macro_rules! matches_comparator {
    () => {
        deps!();
        pub (crate) fn matches_comparator (cmp : & Comparator , ver : & Version) -> bool { matches_impl (cmp , ver) && (ver . pre . is_empty () || pre_is_compatible (cmp , ver)) }
    };
}

matches_comparator!();