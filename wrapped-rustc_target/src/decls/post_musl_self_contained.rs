macro_rules! deps {
    () => {
        CrtObjects!();
    };
}

macro_rules! post_musl_self_contained {
    () => {
        deps!();
        pub (super) fn post_musl_self_contained () -> CrtObjects { new (& [(LinkOutputKind :: DynamicNoPicExe , & ["crtend.o" , "crtn.o"]) , (LinkOutputKind :: DynamicPicExe , & ["crtendS.o" , "crtn.o"]) , (LinkOutputKind :: StaticNoPicExe , & ["crtend.o" , "crtn.o"]) , (LinkOutputKind :: StaticPicExe , & ["crtendS.o" , "crtn.o"]) , (LinkOutputKind :: DynamicDylib , & ["crtendS.o" , "crtn.o"]) , (LinkOutputKind :: StaticDylib , & ["crtendS.o" , "crtn.o"]) ,]) }
    };
}

post_musl_self_contained!()