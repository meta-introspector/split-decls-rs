macro_rules! deps {
    () => {
        ProvenanceMap!();
        BridgeTys!();
        Allocation!();
    };
}

macro_rules! impl_525 {
    () => {
        deps!();
        impl rustc_public_bridge :: bridge :: Allocation < compiler_interface :: BridgeTys > for crate :: ty :: Allocation { fn new < 'tcx > (bytes : Vec < Option < u8 > > , ptrs : Vec < (usize , rustc_middle :: mir :: interpret :: AllocId) > , align : u64 , mutability : rustc_middle :: mir :: Mutability , tables : & mut Tables < 'tcx , compiler_interface :: BridgeTys > , cx : & CompilerCtxt < 'tcx , compiler_interface :: BridgeTys > ,) -> Self { Self { bytes , provenance : ProvenanceMap { ptrs : ptrs . iter () . map (| (i , aid) | (* i , tables . prov (* aid))) . collect () , } , align , mutability : mutability . stable (tables , cx) , } } }
    };
}

impl_525!();