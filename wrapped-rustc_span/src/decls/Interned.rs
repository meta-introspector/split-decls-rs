macro_rules! Interned {
    () => {
        # [derive (Clone , Copy)] struct Interned { index : u32 , }
    };
}

Interned!()