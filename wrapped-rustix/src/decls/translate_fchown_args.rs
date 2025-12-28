macro_rules! deps {
    () => {
        Gid!();
        Uid!();
    };
}

macro_rules! translate_fchown_args {
    () => {
        deps!();
        pub (crate) fn translate_fchown_args (owner : Option < Uid > , group : Option < Gid > ,) -> (c :: uid_t , c :: gid_t) { let ow = match owner { Some (o) => o . as_raw () , None => ! 0 , } ; let gr = match group { Some (g) => g . as_raw () , None => ! 0 , } ; (ow as c :: uid_t , gr as c :: gid_t) }
    };
}

translate_fchown_args!()