macro_rules! deps {
    () => {
        Bin!();
    };
}

macro_rules! BinRegistry {
    () => {
        deps!();
        # [derive (Clone , Debug)] pub (crate) struct BinRegistry { bins : std :: collections :: BTreeMap < String , crate :: schema :: Bin > , fallback : bool , }
    };
}

BinRegistry!();