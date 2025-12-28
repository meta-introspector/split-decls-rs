macro_rules! deps {
    () => {
        DetectionState!();
    };
}

macro_rules! Sha1 {
    () => {
        deps!();
        # [doc = " SHA-1 collision detection hasher state."] # [derive (Clone)] pub struct Sha1 { h : [u32 ; STATE_LEN] , block_len : u64 , detection : Option < DetectionState > , buffer : BlockBuffer < U64 , Eager > , }
    };
}

Sha1!();