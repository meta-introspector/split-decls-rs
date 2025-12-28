macro_rules! is_generic {
    () => {
        fn is_generic < 'tcx > (instance : Instance < 'tcx >) -> bool { instance . args . non_erasable_generics () . next () . is_some () }
    };
}

is_generic!()