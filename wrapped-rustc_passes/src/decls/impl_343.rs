macro_rules! deps {
    () => {
        LocalCollector!();
    };
}

macro_rules! impl_343 {
    () => {
        deps!();
        impl < 'tcx > Visitor < 'tcx > for LocalCollector { fn visit_pat (& mut self , pat : & 'tcx hir :: Pat < 'tcx >) { if let hir :: PatKind :: Binding (_ , hir_id , ..) = pat . kind { self . locals . insert (hir_id) ; } intravisit :: walk_pat (self , pat) ; } }
    };
}

impl_343!()