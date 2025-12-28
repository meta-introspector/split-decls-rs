macro_rules! deps {
    () => {
        Callsite!();
        Interest!();
    };
}

macro_rules! rebuild_callsite_interest {
    () => {
        deps!();
        fn rebuild_callsite_interest (callsite : & 'static dyn Callsite , dispatchers : & dispatchers :: Rebuilder < '_ > ,) { let meta = callsite . metadata () ; let mut interest = None ; dispatchers . for_each (| dispatch | { let this_interest = dispatch . register_callsite (meta) ; interest = match interest . take () { None => Some (this_interest) , Some (that_interest) => Some (that_interest . and (this_interest)) , } }) ; let interest = interest . unwrap_or_else (Interest :: never) ; callsite . set_interest (interest) }
    };
}

rebuild_callsite_interest!()