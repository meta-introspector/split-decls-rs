macro_rules! deps {
    () => {
        ExternalSource!();
        ExternalSourceKind!();
    };
}

macro_rules! impl_295 {
    () => {
        deps!();
        impl ExternalSource { pub fn get_source (& self) -> Option < & str > { match self { ExternalSource :: Foreign { kind : ExternalSourceKind :: Present (src) , .. } => Some (src) , _ => None , } } }
    };
}

impl_295!()