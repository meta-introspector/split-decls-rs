macro_rules! ABI {
    () => {
        # [derive (Debug , Clone , Copy , PartialEq)] enum ABI { ELFv1 , ELFv2 , AIX , }
    };
}

ABI!();