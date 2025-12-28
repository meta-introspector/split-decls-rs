macro_rules! deps {
    () => {
        MaybeStorageLive!();
        Analysis!();
    };
}

macro_rules! impl_167 {
    () => {
        deps!();
        impl < 'a , 'tcx > Analysis < 'tcx > for MaybeStorageLive < 'a > { type Domain = DenseBitSet < Local > ; const NAME : & 'static str = "maybe_storage_live" ; fn bottom_value (& self , body : & Body < 'tcx >) -> Self :: Domain { DenseBitSet :: new_empty (body . local_decls . len ()) } fn initialize_start_block (& self , body : & Body < 'tcx > , state : & mut Self :: Domain) { state . union (& * self . always_live_locals) ; for arg in body . args_iter () { state . insert (arg) ; } } fn apply_primary_statement_effect (& mut self , state : & mut Self :: Domain , stmt : & Statement < 'tcx > , _ : Location ,) { match stmt . kind { StatementKind :: StorageLive (l) => state . gen_ (l) , StatementKind :: StorageDead (l) => state . kill (l) , _ => () , } } }
    };
}

impl_167!();