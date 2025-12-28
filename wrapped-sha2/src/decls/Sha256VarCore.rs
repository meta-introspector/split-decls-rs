macro_rules! deps {
    () => {
        State256!();
    };
}

macro_rules! Sha256VarCore {
    () => {
        deps!();
        # [doc = " Core block-level SHA-256 hasher with variable output size."] # [doc = ""] # [doc = " Supports initialization only for 28 and 32 byte output sizes,"] # [doc = " i.e. 224 and 256 bits respectively."] # [derive (Clone)] pub struct Sha256VarCore { state : consts :: State256 , block_len : u64 , }
    };
}

Sha256VarCore!()