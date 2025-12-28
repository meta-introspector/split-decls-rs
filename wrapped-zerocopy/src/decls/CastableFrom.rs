macro_rules! deps {
    () => {
        Initialized!();
        Uninit!();
    };
}

macro_rules! CastableFrom {
    () => {
        deps!();
        # [doc = " # Safety"] # [doc = ""] # [doc = " `DT: CastableFrom<ST, SV, DV>` is sound if `SV = DV = Uninit` or `SV = DV ="] # [doc = " Initialized`."] pub unsafe trait CastableFrom < ST : ? Sized , SV , DV > { }
    };
}

CastableFrom!()