macro_rules! FlockOffsetType {
    () => {
        # [doc = " `F_SEEK*` constants for use with [`fcntl_getlk`]."] # [doc = ""] # [doc = " [`fcntl_getlk`]: crate::process::fcntl_getlk()"] # [cfg (not (target_os = "horizon"))] # [derive (Clone , Copy , Debug , PartialEq , Eq)] # [repr (i16)] pub enum FlockOffsetType { # [doc = " `F_SEEK_SET`"] Set = c :: SEEK_SET as _ , # [doc = " `F_SEEK_CUR`"] Current = c :: SEEK_CUR as _ , # [doc = " `F_SEEK_END`"] End = c :: SEEK_END as _ , }
    };
}

FlockOffsetType!()