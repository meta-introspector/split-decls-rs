macro_rules! deps {
    () => {
        State512!();
    };
}

macro_rules! Sha512VarCore {
    () => {
        deps!();
        # [doc = " Core block-level SHA-512 hasher with variable output size."] # [doc = ""] # [doc = " Supports initialization only for 28, 32, 48, and 64 byte output sizes,"] # [doc = " i.e. 224, 256, 384, and 512 bits respectively."] # [derive (Clone)] pub struct Sha512VarCore { state : consts :: State512 , block_len : u128 , }
    };
}

Sha512VarCore!()