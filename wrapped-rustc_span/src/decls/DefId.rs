macro_rules! deps {
    () => {
        LocalDefId!();
    };
}

macro_rules! DefId {
    () => {
        deps!();
        # [doc = " A `DefId` identifies a particular *definition*, by combining a crate"] # [doc = " index and a def index."] # [doc = ""] # [doc = " You can create a `DefId` from a `LocalDefId` using `local_def_id.to_def_id()`."] # [derive (Clone , PartialEq , Eq , Copy)] # [cfg_attr (not (target_pointer_width = "64") , derive (Hash))] # [repr (C)] # [rustc_pass_by_value] pub struct DefId { # [cfg (not (all (target_pointer_width = "64" , target_endian = "big")))] pub index : DefIndex , pub krate : CrateNum , # [cfg (all (target_pointer_width = "64" , target_endian = "big"))] pub index : DefIndex , }
    };
}

DefId!();