macro_rules! deps {
    () => {
        TargetMetadata!();
        ToJson!();
    };
}

macro_rules! impl_304 {
    () => {
        deps!();
        impl ToJson for TargetMetadata { fn to_json (& self) -> Json { json ! ({ "description" : self . description , "tier" : self . tier , "host_tools" : self . host_tools , "std" : self . std , }) } }
    };
}

impl_304!()