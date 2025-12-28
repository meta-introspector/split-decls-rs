macro_rules! deps {
    () => {
        Interner!();
        Region!();
    };
}

macro_rules! shift_region {
    () => {
        deps!();
        pub fn shift_region < I : Interner > (cx : I , region : I :: Region , amount : u32) -> I :: Region { match region . kind () { ty :: ReBound (debruijn , br) if amount > 0 => { Region :: new_bound (cx , debruijn . shifted_in (amount) , br) } _ => region , } }
    };
}

shift_region!()