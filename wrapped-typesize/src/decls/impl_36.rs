macro_rules! deps {
    () => {
        EntryRef!();
    };
}

macro_rules! impl_36 {
    () => {
        deps!();
        # [cfg (feature = "dashmap")] impl < K : Eq + Hash , V , S : BuildHasher + Clone > crate :: map :: EntryRef < K , V > for mini_moka :: sync :: EntryRef < '_ , K , V , S > { fn get_ref (& self) -> (& K , & V) { self . pair () } }
    };
}

impl_36!();