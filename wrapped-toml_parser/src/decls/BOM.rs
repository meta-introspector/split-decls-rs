macro_rules! BOM {
    () => {
        const BOM : & [u8] = b"\xEF\xBB\xBF" ;
    };
}

BOM!()