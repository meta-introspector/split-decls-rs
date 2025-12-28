macro_rules! PartiallyInterned {
    () => {
        # [derive (Clone , Copy)] struct PartiallyInterned { index : u32 , ctxt : u16 , }
    };
}

PartiallyInterned!()