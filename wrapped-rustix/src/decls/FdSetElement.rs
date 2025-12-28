macro_rules! FdSetElement {
    () => {
        # [doc = " Storage element type for use with [`select`]."] # [cfg (target_os = "wasi")] # [repr (transparent)] # [derive (Copy , Clone , Default)] pub struct FdSetElement (pub (crate) usize) ;
    };
}

FdSetElement!();