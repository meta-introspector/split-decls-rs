macro_rules! deps {
    () => {
        BridgeTys!();
        Stable!();
    };
}

macro_rules! impl_158 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for mir :: interpret :: GlobalAlloc < 'tcx > { type T = GlobalAlloc ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { match self { mir :: interpret :: GlobalAlloc :: Function { instance , .. } => { GlobalAlloc :: Function (instance . stable (tables , cx)) } mir :: interpret :: GlobalAlloc :: VTable (ty , dyn_ty) => { GlobalAlloc :: VTable (ty . stable (tables , cx) , dyn_ty . principal () . stable (tables , cx)) } mir :: interpret :: GlobalAlloc :: Static (def) => { GlobalAlloc :: Static (tables . static_def (* def)) } mir :: interpret :: GlobalAlloc :: Memory (alloc) => { GlobalAlloc :: Memory (alloc . stable (tables , cx)) } mir :: interpret :: GlobalAlloc :: TypeId { ty } => { GlobalAlloc :: TypeId { ty : ty . stable (tables , cx) } } } } }
    };
}

impl_158!();