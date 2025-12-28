macro_rules! deps {
    () => {
        CompilerCtxt!();
        Error!();
        Bridge!();
        Allocation!();
    };
}

macro_rules! try_new_slice {
    () => {
        deps!();
        pub fn try_new_slice < 'tcx , B : Bridge > (layout : TyAndLayout < 'tcx , Ty < 'tcx > > , alloc_id : AllocId , meta : u64 , cx : & CompilerCtxt < 'tcx , B > ,) -> Result < Allocation , B :: Error > { let ptr = Pointer :: new (alloc_id . into () , Size :: ZERO) ; let scalar_ptr = Scalar :: from_pointer (ptr , & cx . tcx) ; let scalar_meta : Scalar = Scalar :: from_target_usize (meta , & cx . tcx) ; let mut allocation = Allocation :: new (layout . size , layout . align . abi , AllocInit :: Uninit , ()) ; let ptr_size = cx . tcx . data_layout . pointer_size () ; allocation . write_scalar (& cx . tcx , alloc_range (Size :: ZERO , ptr_size) , scalar_ptr) . map_err (| e | B :: Error :: from_internal (e)) ? ; allocation . write_scalar (& cx . tcx , alloc_range (ptr_size , scalar_meta . size ()) , scalar_meta) . map_err (| e | B :: Error :: from_internal (e)) ? ; Ok (allocation) }
    };
}

try_new_slice!();