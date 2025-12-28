macro_rules! deps {
    () => {
        CrtObjects!();
    };
}

macro_rules! pre_mingw_self_contained {
    () => {
        deps!();
        pub (super) fn pre_mingw_self_contained () -> CrtObjects { new (& [(LinkOutputKind :: DynamicNoPicExe , & ["crt2.o" , "rsbegin.o"]) , (LinkOutputKind :: DynamicPicExe , & ["crt2.o" , "rsbegin.o"]) , (LinkOutputKind :: StaticNoPicExe , & ["crt2.o" , "rsbegin.o"]) , (LinkOutputKind :: StaticPicExe , & ["crt2.o" , "rsbegin.o"]) , (LinkOutputKind :: DynamicDylib , & ["dllcrt2.o" , "rsbegin.o"]) , (LinkOutputKind :: StaticDylib , & ["dllcrt2.o" , "rsbegin.o"]) ,]) }
    };
}

pre_mingw_self_contained!()