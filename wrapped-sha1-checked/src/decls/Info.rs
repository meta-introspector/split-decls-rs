macro_rules! deps {
    () => {
        Testt!();
    };
}

macro_rules! Info {
    () => {
        deps!();
        # [doc = " Disturbance Vector (DV)."] pub (crate) struct Info { # [doc = " The step to do the recompression from for collision detection."] pub (crate) testt : Testt , # [doc = " Defines the bit to check for each DV in the dvmask returned by [`ubc_check`]."] pub (crate) maskb : i32 , # [doc = " The expanded message block XOR-difference defined by the DV."] pub (crate) dm : [u32 ; 80] , }
    };
}

Info!()