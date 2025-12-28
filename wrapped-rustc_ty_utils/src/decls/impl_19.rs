macro_rules! deps {
    () => {
        RPITVisitor!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl < 'tcx > Visitor < 'tcx > for RPITVisitor < '_ , 'tcx > { fn visit_opaque_ty (& mut self , opaque : & 'tcx hir :: OpaqueTy < 'tcx >) -> Self :: Result { self . synthetics . push (associated_type_for_impl_trait_in_trait (self . tcx , opaque . def_id , self . data , & mut self . disambiguator ,)) ; intravisit :: walk_opaque_ty (self , opaque) } }
    };
}

impl_19!();