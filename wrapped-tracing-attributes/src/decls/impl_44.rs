macro_rules! deps {
    () => {
        ImplTraitEraser!();
    };
}

macro_rules! impl_44 {
    () => {
        deps!();
        impl VisitMut for ImplTraitEraser { fn visit_type_mut (& mut self , t : & mut Type) { if let Type :: ImplTrait (..) = t { * t = syn :: TypeInfer { underscore_token : Token ! [_] (t . span ()) , } . into () ; } else { syn :: visit_mut :: visit_type_mut (self , t) ; } } }
    };
}

impl_44!()