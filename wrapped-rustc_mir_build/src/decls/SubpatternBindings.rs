macro_rules! deps {
    () => {
        Binding!();
    };
}

macro_rules! SubpatternBindings {
    () => {
        deps!();
        # [derive (Debug , Clone)] enum SubpatternBindings < 'tcx > { # [doc = " A single binding."] One (Binding < 'tcx >) , # [doc = " Holds the place for an or-pattern's bindings. This ensures their drops are scheduled in the"] # [doc = " order the primary bindings appear. See rust-lang/rust#142163 for more information."] FromOrPattern , }
    };
}

SubpatternBindings!()