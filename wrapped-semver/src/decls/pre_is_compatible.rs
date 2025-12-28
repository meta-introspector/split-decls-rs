macro_rules! deps {
    () => {
        Comparator!();
        Version!();
    };
}

macro_rules! pre_is_compatible {
    () => {
        deps!();
        fn pre_is_compatible (cmp : & Comparator , ver : & Version) -> bool { cmp . major == ver . major && cmp . minor == Some (ver . minor) && cmp . patch == Some (ver . patch) && ! cmp . pre . is_empty () }
    };
}

pre_is_compatible!();