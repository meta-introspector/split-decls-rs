macro_rules! RngSeed {
    () => {
        # [doc = " A seed for random number generation."] # [doc = ""] # [doc = " In order to make certain functions within a runtime deterministic, a seed"] # [doc = " can be specified at the time of creation."] # [allow (unreachable_pub)] # [derive (Clone , Debug)] pub struct RngSeed { s : u32 , r : u32 , }
    };
}

RngSeed!();