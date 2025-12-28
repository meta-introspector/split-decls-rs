macro_rules! triple {
    () => {
        # [doc = " Reads the triple of [`Version`], [`Channel`], and [`Date`] of the installed"] # [doc = " or running `rustc`."] # [doc = ""] # [doc = " If any attribute cannot be determined (see the [top-level"] # [doc = " documentation](crate)), returns `None`."] # [doc = ""] # [doc = " To obtain only one of three attributes, use [`Version::read()`],"] # [doc = " [`Channel::read()`], or [`Date::read()`]."] pub fn triple () -> Option < (Version , Channel , Date) > { let (version_str , date_str) = match get_version_and_date () { Some ((Some (version) , Some (date))) => (version , date) , _ => return None , } ; match Version :: parse (& version_str) { Some (version) => match Channel :: parse (& version_str) { Some (channel) => match Date :: parse (& date_str) { Some (date) => Some ((version , channel , date)) , _ => None , } , _ => None , } , _ => None , } }
    };
}

triple!()