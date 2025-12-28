macro_rules! DateTime {
    () => {
        # [doc = " Representation of a moment in time."] # [doc = ""] # [doc = " Zip files use an old format from DOS to store timestamps,"] # [doc = " with its own set of peculiarities."] # [doc = " For example, it has a resolution of 2 seconds!"] # [doc = ""] # [doc = " A [`DateTime`] can be stored directly in a zipfile with [`FileOptions::last_modified_time`],"] # [doc = " or read from one with [`ZipFile::last_modified`](crate::read::ZipFile::last_modified)."] # [doc = ""] # [doc = " # Warning"] # [doc = ""] # [doc = " Because there is no timezone associated with the [`DateTime`], they should ideally only"] # [doc = " be used for user-facing descriptions."] # [doc = ""] # [doc = " Modern zip files store more precise timestamps; see [`crate::extra_fields::ExtendedTimestamp`]"] # [doc = " for details."] # [derive (Clone , Copy , Eq , Hash , PartialEq)] pub struct DateTime { datepart : u16 , timepart : u16 , }
    };
}

DateTime!()