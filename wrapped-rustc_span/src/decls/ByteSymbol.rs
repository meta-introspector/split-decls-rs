macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! ByteSymbol {
    () => {
        deps!();
        # [doc = " Like `Symbol`, but for byte strings. `ByteSymbol` is used less widely, so"] # [doc = " it has fewer operations defined than `Symbol`."] # [derive (Clone , Copy , PartialEq , Eq , PartialOrd , Ord , Hash)] pub struct ByteSymbol (SymbolIndex) ;
    };
}

ByteSymbol!()