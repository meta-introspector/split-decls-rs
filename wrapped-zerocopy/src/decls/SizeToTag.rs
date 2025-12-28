macro_rules! SizeToTag {
    () => {
        # [doc = " An alias for the unsigned integer of the given size in bytes."] # [doc (hidden)] pub type SizeToTag < const SIZE : usize > = < () as size_to_tag :: SizeToTag < SIZE > > :: Tag ;
    };
}

SizeToTag!();