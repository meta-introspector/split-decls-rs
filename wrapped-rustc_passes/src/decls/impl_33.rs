macro_rules! deps {
    () => {
        ImplsOrderVisitor!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        impl < 'tcx > Visitor < 'tcx > for ImplsOrderVisitor < 'tcx > { type NestedFilter = nested_filter :: All ; fn maybe_tcx (& mut self) -> Self :: MaybeTyCtxt { self . tcx } fn visit_item (& mut self , item : & 'tcx hir :: Item < 'tcx >) { if let hir :: ItemKind :: Impl (impl_) = item . kind && impl_ . of_trait . is_none () && self . tcx . is_exportable (item . owner_id . def_id . to_def_id ()) { self . order . insert (item . owner_id . def_id . to_def_id () , self . order . len ()) ; } intravisit :: walk_item (self , item) ; } }
    };
}

impl_33!();