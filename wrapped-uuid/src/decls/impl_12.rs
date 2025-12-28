macro_rules! deps {
    () => {
        Builder!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        # [doc (hidden)] impl Builder { # [deprecated (since = "1.10.0" , note = "use `Builder::from_gregorian_timestamp(ticks, counter, node_id)`")] pub const fn from_rfc4122_timestamp (ticks : u64 , counter : u16 , node_id : & [u8 ; 6]) -> Self { Builder :: from_gregorian_timestamp (ticks , counter , node_id) } # [deprecated (since = "1.10.0" , note = "use `Builder::from_sorted_gregorian_timestamp(ticks, counter, node_id)`")] pub const fn from_sorted_rfc4122_timestamp (ticks : u64 , counter : u16 , node_id : & [u8 ; 6] ,) -> Self { Builder :: from_sorted_gregorian_timestamp (ticks , counter , node_id) } }
    };
}

impl_12!()