macro_rules! deps {
    () => {
        BridgeTys!();
        Stable!();
    };
}

macro_rules! impl_123 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for mir :: Body < 'tcx > { type T = crate :: mir :: Body ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { crate :: mir :: Body :: new (self . basic_blocks . iter () . map (| block | crate :: mir :: BasicBlock { terminator : block . terminator () . stable (tables , cx) , statements : block . statements . iter () . map (| statement | statement . stable (tables , cx)) . collect () , }) . collect () , self . local_decls . iter () . map (| decl | crate :: mir :: LocalDecl { ty : decl . ty . stable (tables , cx) , span : decl . source_info . span . stable (tables , cx) , mutability : decl . mutability . stable (tables , cx) , }) . collect () , self . arg_count , self . var_debug_info . iter () . map (| info | info . stable (tables , cx)) . collect () , self . spread_arg . stable (tables , cx) , self . span . stable (tables , cx) ,) } }
    };
}

impl_123!();