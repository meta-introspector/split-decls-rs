macro_rules! deps {
    () => {
        Allocation!();
        CompilerCtxt!();
        Tables!();
        Bridge!();
    };
}

macro_rules! allocation_filter {
    () => {
        deps!();
        # [doc = " Creates an `Allocation` only from information within the `AllocRange`."] pub fn allocation_filter < 'tcx , B : Bridge > (alloc : & rustc_middle :: mir :: interpret :: Allocation , alloc_range : AllocRange , tables : & mut Tables < 'tcx , B > , cx : & CompilerCtxt < 'tcx , B > ,) -> B :: Allocation { let mut bytes : Vec < Option < u8 > > = alloc . inspect_with_uninit_and_ptr_outside_interpreter (alloc_range . start . bytes_usize () .. alloc_range . end () . bytes_usize () ,) . iter () . copied () . map (Some) . collect () ; for (i , b) in bytes . iter_mut () . enumerate () { if ! alloc . init_mask () . get (Size :: from_bytes (i + alloc_range . start . bytes_usize ())) { * b = None ; } } let mut ptrs = Vec :: new () ; for (offset , prov) in alloc . provenance () . ptrs () . iter () . filter (| a | a . 0 >= alloc_range . start && a . 0 <= alloc_range . end ()) { ptrs . push ((offset . bytes_usize () - alloc_range . start . bytes_usize () , prov . alloc_id ())) ; } B :: Allocation :: new (bytes , ptrs , alloc . align . bytes () , alloc . mutability , tables , cx) }
    };
}

allocation_filter!()