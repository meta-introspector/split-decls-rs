macro_rules! deps {
    () => {
        Entry!();
    };
}

macro_rules! Slab {
    () => {
        deps!();
        # [doc = " Pre-allocated storage for a uniform data type"] # [doc = ""] # [doc = " See the [module documentation] for more details."] # [doc = ""] # [doc = " [module documentation]: index.html"] pub struct Slab < T > { entries : Vec < Entry < T > > , len : usize , next : usize , }
    };
}

Slab!();