macro_rules! deps {
    () => {
        Analysis!();
        MaybeStorageDead!();
    };
}

macro_rules! impl_170 {
    () => {
        deps!();
        impl < 'a , 'tcx > Analysis < 'tcx > for MaybeStorageDead < 'a > { type Domain = DenseBitSet < Local > ; const NAME : & 'static str = "maybe_storage_dead" ; fn bottom_value (& self , body : & Body < 'tcx >) -> Self :: Domain { DenseBitSet :: new_empty (body . local_decls . len ()) } fn initialize_start_block (& self , body : & Body < 'tcx > , state : & mut Self :: Domain) { assert_eq ! (body . local_decls . len () , self . always_live_locals . domain_size ()) ; for local in body . vars_and_temps_iter () { if ! self . always_live_locals . contains (local) { state . insert (local) ; } } } fn apply_primary_statement_effect (& mut self , state : & mut Self :: Domain , stmt : & Statement < 'tcx > , _ : Location ,) { match stmt . kind { StatementKind :: StorageLive (l) => state . kill (l) , StatementKind :: StorageDead (l) => state . gen_ (l) , _ => () , } } }
    };
}

impl_170!()