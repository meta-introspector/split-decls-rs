macro_rules! deps {
    () => {
        RefutableFlag!();
        BindingsWithVariantName!();
        Binding!();
        MatchVisitor!();
    };
}

macro_rules! check_for_bindings_named_same_as_variants {
    () => {
        deps!();
        fn check_for_bindings_named_same_as_variants (cx : & MatchVisitor < '_ , '_ > , pat : & Pat < '_ > , rf : RefutableFlag ,) { if let PatKind :: Binding { name , mode : BindingMode (ByRef :: No , Mutability :: Not) , subpattern : None , ty , .. } = pat . kind && let ty :: Adt (edef , _) = ty . peel_refs () . kind () && edef . is_enum () && edef . variants () . iter () . any (| variant | variant . name == name && variant . ctor_kind () == Some (CtorKind :: Const)) { let variant_count = edef . variants () . len () ; let ty_path = with_no_trimmed_paths ! (cx . tcx . def_path_str (edef . did ())) ; cx . tcx . emit_node_span_lint (BINDINGS_WITH_VARIANT_NAME , cx . lint_level , pat . span , BindingsWithVariantName { suggestion : if rf == Refutable || variant_count == 1 { Some (pat . span) } else { None } , ty_path , name : Ident :: new (name , pat . span) , } ,) } }
    };
}

check_for_bindings_named_same_as_variants!();