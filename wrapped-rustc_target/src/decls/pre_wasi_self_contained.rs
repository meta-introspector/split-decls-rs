macro_rules! deps {
    () => {
        CrtObjects!();
    };
}

macro_rules! pre_wasi_self_contained {
    () => {
        deps!();
        pub (super) fn pre_wasi_self_contained () -> CrtObjects { new (& [(LinkOutputKind :: DynamicNoPicExe , & ["crt1-command.o"]) , (LinkOutputKind :: DynamicPicExe , & ["crt1-command.o"]) , (LinkOutputKind :: StaticNoPicExe , & ["crt1-command.o"]) , (LinkOutputKind :: StaticPicExe , & ["crt1-command.o"]) , (LinkOutputKind :: WasiReactorExe , & ["crt1-reactor.o"]) ,]) }
    };
}

pre_wasi_self_contained!();