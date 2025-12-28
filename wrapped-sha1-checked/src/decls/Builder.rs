macro_rules! Builder {
    () => {
        # [doc = " Builder for collision detection configuration."] # [derive (Clone)] pub struct Builder { detect_collision : bool , safe_hash : bool , ubc_check : bool , reduced_round_collision : bool , }
    };
}

Builder!();