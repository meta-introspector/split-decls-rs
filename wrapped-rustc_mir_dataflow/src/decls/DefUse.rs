macro_rules! DefUse {
    () => {
        # [derive (Eq , PartialEq , Clone)] pub enum DefUse { # [doc = " Full write to the local."] Def , # [doc = " Read of any part of the local."] Use , # [doc = " Partial write to the local."] PartialWrite , # [doc = " Non-use, like debuginfo."] NonUse , }
    };
}

DefUse!();