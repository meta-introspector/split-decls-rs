macro_rules! Qualifiers {
    () => {
        # [derive (PartialOrd , PartialEq)] enum Qualifiers { None , Async , Unsafe , Extern , Abi , }
    };
}

Qualifiers!()