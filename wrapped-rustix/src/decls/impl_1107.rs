macro_rules! deps {
    () => {
        FlockOffsetType!();
        FlockType!();
        Flock!();
    };
}

macro_rules! impl_1107 {
    () => {
        deps!();
        # [cfg (not (target_os = "horizon"))] impl From < FlockType > for Flock { fn from (value : FlockType) -> Self { Self { start : 0 , length : 0 , pid : None , typ : value , offset_type : FlockOffsetType :: Set , } } }
    };
}

impl_1107!()