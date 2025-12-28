macro_rules! ChangeSelfPath {
    () => {
        pub (crate) struct ChangeSelfPath < 'a > { self_ty : & 'a syn :: Type , trait_ : Option < (& 'a syn :: Path , & 'a HashSet < syn :: Ident >) > , }
    };
}

ChangeSelfPath!();