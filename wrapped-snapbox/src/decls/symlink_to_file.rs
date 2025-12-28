macro_rules! deps {
    () => {
        Error!();
        Result!();
    };
}

macro_rules! symlink_to_file {
    () => {
        deps!();
        # [cfg (not (windows))] fn symlink_to_file (link : & std :: path :: Path , target : & std :: path :: Path) -> Result < () , std :: io :: Error > { std :: os :: unix :: fs :: symlink (target , link) }
    };
}

symlink_to_file!()