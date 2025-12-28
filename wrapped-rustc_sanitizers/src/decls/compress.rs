macro_rules! deps {
    () => {
        DictKey!();
    };
}

macro_rules! compress {
    () => {
        deps!();
        # [doc = " Substitutes a component if found in the substitution dictionary (see"] # [doc = " <https://itanium-cxx-abi.github.io/cxx-abi/abi.html#mangling-compression>)."] fn compress < 'tcx > (dict : & mut FxHashMap < DictKey < 'tcx > , usize > , key : DictKey < 'tcx > , comp : & mut String ,) { match dict . get (& key) { Some (num) => { comp . clear () ; let _ = write ! (comp , "S{}_" , to_seq_id (* num)) ; } None => { dict . insert (key , dict . len ()) ; } } }
    };
}

compress!();