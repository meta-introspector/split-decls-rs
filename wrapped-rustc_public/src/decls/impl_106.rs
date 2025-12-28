macro_rules! deps {
    () => {
        PassMode!();
        BridgeTys!();
        Stable!();
    };
}

macro_rules! impl_106 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for callconv :: PassMode { type T = PassMode ; fn stable (& self , _ : & mut Tables < '_ , BridgeTys > , _ : & CompilerCtxt < '_ , BridgeTys >) -> Self :: T { match self { callconv :: PassMode :: Ignore => PassMode :: Ignore , callconv :: PassMode :: Direct (attr) => PassMode :: Direct (opaque (attr)) , callconv :: PassMode :: Pair (first , second) => { PassMode :: Pair (opaque (first) , opaque (second)) } callconv :: PassMode :: Cast { pad_i32 , cast } => { PassMode :: Cast { pad_i32 : * pad_i32 , cast : opaque (cast) } } callconv :: PassMode :: Indirect { attrs , meta_attrs , on_stack } => PassMode :: Indirect { attrs : opaque (attrs) , meta_attrs : opaque (meta_attrs) , on_stack : * on_stack , } , } } }
    };
}

impl_106!()