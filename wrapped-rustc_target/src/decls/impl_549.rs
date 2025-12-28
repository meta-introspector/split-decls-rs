macro_rules! deps {
    () => {
        TargetTuple!();
    };
}

macro_rules! impl_549 {
    () => {
        deps!();
        impl < D : Decoder > Decodable < D > for TargetTuple { fn decode (d : & mut D) -> Self { match d . read_u8 () { 0 => TargetTuple :: TargetTuple (d . read_str () . to_owned ()) , 1 => TargetTuple :: TargetJson { path_for_rustdoc : PathBuf :: new () , tuple : d . read_str () . to_owned () , contents : d . read_str () . to_owned () , } , _ => { panic ! ("invalid enum variant tag while decoding `TargetTuple`, expected 0..2") ; } } } }
    };
}

impl_549!();