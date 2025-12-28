macro_rules! impl_498 {
    () => {
        impl LinkOutputKind { pub fn can_link_dylib (self) -> bool { match self { LinkOutputKind :: StaticNoPicExe | LinkOutputKind :: StaticPicExe => false , LinkOutputKind :: DynamicNoPicExe | LinkOutputKind :: DynamicPicExe | LinkOutputKind :: DynamicDylib | LinkOutputKind :: StaticDylib | LinkOutputKind :: WasiReactorExe => true , } } }
    };
}

impl_498!();