macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! copy_recursively_into_existing_dir {
    () => {
        deps!();
        # [doc = " A utility to copy the entire contents of `src_dir` into `dst_dir`."] pub fn copy_recursively_into_existing_dir (src_dir : impl AsRef < Path > , dst_dir : impl AsRef < Path >) -> std :: io :: Result < () > { fs_extra :: copy_items (& std :: fs :: read_dir (src_dir) ? . map (| e | e . map (| e | e . path ())) . collect :: < std :: result :: Result < Vec < _ > , _ > > () ? , dst_dir , & fs_extra :: dir :: CopyOptions { overwrite : false , skip_exist : false , copy_inside : false , content_only : false , .. Default :: default () } ,) . map_err (std :: io :: Error :: other) ? ; Ok (()) }
    };
}

copy_recursively_into_existing_dir!()