macro_rules! deps {
    () => {
        Name!();
    };
}

macro_rules! StructVariant {
    () => {
        deps!();
        enum StructVariant < 'a > { ExternallyTagged { variant_index : u32 , variant_name : & 'a Name , } , InternallyTagged { tag : & 'a str , variant_name : & 'a Name , } , Untagged , }
    };
}

StructVariant!();