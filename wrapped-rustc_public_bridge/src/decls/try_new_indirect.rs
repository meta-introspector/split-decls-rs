macro_rules! deps {
    () => {
        Bridge!();
        CompilerCtxt!();
    };
}

macro_rules! try_new_indirect {
    () => {
        deps!();
        pub fn try_new_indirect < 'tcx , B : Bridge > (alloc_id : AllocId , cx : & CompilerCtxt < 'tcx , B > ,) -> ConstAllocation < 'tcx > { let alloc = cx . tcx . global_alloc (alloc_id) . unwrap_memory () ; alloc }
    };
}

try_new_indirect!()