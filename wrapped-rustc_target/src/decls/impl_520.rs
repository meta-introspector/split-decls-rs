macro_rules! impl_520 {
    () => {
        impl BinaryFormat { # [doc = " Returns [`object::BinaryFormat`] for given `BinaryFormat`"] pub fn to_object (& self) -> object :: BinaryFormat { match self { Self :: Coff => object :: BinaryFormat :: Coff , Self :: Elf => object :: BinaryFormat :: Elf , Self :: MachO => object :: BinaryFormat :: MachO , Self :: Wasm => object :: BinaryFormat :: Wasm , Self :: Xcoff => object :: BinaryFormat :: Xcoff , } } }
    };
}

impl_520!();