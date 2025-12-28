macro_rules! deps {
    () => {
        CrtObjects!();
    };
}

macro_rules! all {
    () => {
        deps!();
        pub (super) fn all (obj : & 'static str) -> CrtObjects { new (& [(LinkOutputKind :: DynamicNoPicExe , & [obj]) , (LinkOutputKind :: DynamicPicExe , & [obj]) , (LinkOutputKind :: StaticNoPicExe , & [obj]) , (LinkOutputKind :: StaticPicExe , & [obj]) , (LinkOutputKind :: DynamicDylib , & [obj]) , (LinkOutputKind :: StaticDylib , & [obj]) ,]) }
    };
}

all!();