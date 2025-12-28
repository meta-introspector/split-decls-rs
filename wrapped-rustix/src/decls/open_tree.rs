macro_rules! deps {
    () => {
        Arg!();
        Result!();
    };
}

macro_rules! open_tree {
    () => {
        deps!();
        # [doc = " `open_tree(dfd, filename, flags)`"] # [doc = ""] # [doc = " # References"] # [doc = "  - [Unfinished draft]"] # [doc = ""] # [doc = " [Unfinished draft]: https://github.com/sunfishcode/linux-mount-api-documentation/blob/main/open_tree.md"] # [inline] pub fn open_tree < Path : path :: Arg , Fd : AsFd > (dfd : Fd , filename : Path , flags : OpenTreeFlags ,) -> io :: Result < OwnedFd > { let dfd = dfd . as_fd () ; filename . into_with_c_str (| filename | backend :: mount :: syscalls :: open_tree (dfd , filename , flags)) }
    };
}

open_tree!();