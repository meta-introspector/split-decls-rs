macro_rules! deps {
    () => {
        Map!();
    };
}

macro_rules! DenseLocationMap {
    () => {
        deps!();
        # [doc = " Maps between a `Location` and a `PointIndex` (and vice versa)."] pub struct DenseLocationMap { # [doc = " For each basic block, how many points are contained within?"] statements_before_block : IndexVec < BasicBlock , usize > , # [doc = " Map backward from each point to the basic block that it"] # [doc = " belongs to."] basic_blocks : IndexVec < PointIndex , BasicBlock > , num_points : usize , }
    };
}

DenseLocationMap!()