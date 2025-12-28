macro_rules! DetectionState {
    () => {
        # [doc = " The internal state used to do collision detection."] # [derive (Clone , Debug)] struct DetectionState { safe_hash : bool , ubc_check : bool , reduced_round_collision : bool , # [doc = " Has a collision been detected?"] found_collision : bool , ihv1 : [u32 ; 5] , ihv2 : [u32 ; 5] , m1 : [u32 ; 80] , m2 : [u32 ; 80] , # [doc = " Stores past states, for faster recompression."] state_58 : [u32 ; 5] , state_65 : [u32 ; 5] , }
    };
}

DetectionState!();