macro_rules! deps {
    () => {
        Table!();
        Flags!();
    };
}

macro_rules! Mode {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy)] # [repr (u8)] pub enum Mode { Head , Flags , Time , Os , ExLen , Extra , Name , Comment , HCrc , Sync , Mem , Length , Type , TypeDo , Stored , CopyBlock , Check , Len_ , Len , Lit , LenExt , Dist , DistExt , Match , Table , LenLens , CodeLens , DictId , Dict , Done , Bad , }
    };
}

Mode!()