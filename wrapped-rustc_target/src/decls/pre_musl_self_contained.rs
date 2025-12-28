macro_rules! deps {
    () => {
        CrtObjects!();
    };
}

macro_rules! pre_musl_self_contained {
    () => {
        deps!();
        pub (super) fn pre_musl_self_contained () -> CrtObjects { new (& [(LinkOutputKind :: DynamicNoPicExe , & ["crt1.o" , "crti.o" , "crtbegin.o"]) , (LinkOutputKind :: DynamicPicExe , & ["Scrt1.o" , "crti.o" , "crtbeginS.o"]) , (LinkOutputKind :: StaticNoPicExe , & ["crt1.o" , "crti.o" , "crtbegin.o"]) , (LinkOutputKind :: StaticPicExe , & ["rcrt1.o" , "crti.o" , "crtbeginS.o"]) , (LinkOutputKind :: DynamicDylib , & ["crti.o" , "crtbeginS.o"]) , (LinkOutputKind :: StaticDylib , & ["crti.o" , "crtbeginS.o"]) ,]) }
    };
}

pre_musl_self_contained!()