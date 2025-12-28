macro_rules! deps {
    () => {
        B0!();
        Bit!();
        GetBitOut!();
        Same!();
        B1!();
    };
}

macro_rules! test_get_bit {
    () => {
        deps!();
        # [test] fn test_get_bit () { use crate :: consts :: * ; use crate :: Same ; type T1 = < GetBitOut < U2 , U0 > as Same < B0 > > :: Output ; type T2 = < GetBitOut < U2 , U1 > as Same < B1 > > :: Output ; type T3 = < GetBitOut < U2 , U2 > as Same < B0 > > :: Output ; < T1 as Bit > :: to_bool () ; < T2 as Bit > :: to_bool () ; < T3 as Bit > :: to_bool () ; }
    };
}

test_get_bit!()