macro_rules! PartialEqImplStatus {
    () => {
        # [derive (Debug)] struct PartialEqImplStatus { has_impl : bool , is_derived : bool , structural_partial_eq : bool , non_blanket_impl : Option < DefId > , }
    };
}

PartialEqImplStatus!()