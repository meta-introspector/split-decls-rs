macro_rules! deps {
    () => {
        MovePathLookup!();
        MovePath!();
        LocationMap!();
        MoveOut!();
        Init!();
    };
}

macro_rules! MoveData {
    () => {
        deps!();
        # [derive (Debug)] pub struct MoveData < 'tcx > { pub move_paths : IndexVec < MovePathIndex , MovePath < 'tcx > > , pub moves : IndexVec < MoveOutIndex , MoveOut > , # [doc = " Each Location `l` is mapped to the MoveOut's that are effects"] # [doc = " of executing the code at `l`. (There can be multiple MoveOut's"] # [doc = " for a given `l` because each MoveOut is associated with one"] # [doc = " particular path being moved.)"] pub loc_map : LocationMap < SmallVec < [MoveOutIndex ; 4] > > , pub path_map : IndexVec < MovePathIndex , SmallVec < [MoveOutIndex ; 4] > > , pub rev_lookup : MovePathLookup < 'tcx > , pub inits : IndexVec < InitIndex , Init > , # [doc = " Each Location `l` is mapped to the Inits that are effects"] # [doc = " of executing the code at `l`."] pub init_loc_map : LocationMap < SmallVec < [InitIndex ; 4] > > , pub init_path_map : IndexVec < MovePathIndex , SmallVec < [InitIndex ; 4] > > , }
    };
}

MoveData!();