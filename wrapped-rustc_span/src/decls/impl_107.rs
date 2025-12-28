macro_rules! deps {
    () => {
        LocalDefId!();
        DefId!();
    };
}

macro_rules! impl_107 {
    () => {
        deps!();
        impl DefId { # [doc = " Makes a local `DefId` from the given `DefIndex`."] # [inline] pub fn local (index : DefIndex) -> DefId { DefId { krate : LOCAL_CRATE , index } } # [doc = " Returns whether the item is defined in the crate currently being compiled."] # [inline] pub fn is_local (self) -> bool { self . krate == LOCAL_CRATE } # [inline] pub fn as_local (self) -> Option < LocalDefId > { self . is_local () . then (| | LocalDefId { local_def_index : self . index }) } # [inline] # [track_caller] pub fn expect_local (self) -> LocalDefId { match self . as_local () { Some (local_def_id) => local_def_id , None => panic ! ("DefId::expect_local: `{self:?}` isn't local") , } } # [inline] pub fn is_crate_root (self) -> bool { self . index == CRATE_DEF_INDEX } # [inline] pub fn as_crate_root (self) -> Option < CrateNum > { self . is_crate_root () . then_some (self . krate) } # [inline] pub fn is_top_level_module (self) -> bool { self . is_local () && self . is_crate_root () } }
    };
}

impl_107!()