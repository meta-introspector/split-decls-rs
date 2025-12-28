macro_rules! Tag {
    () => {
        # [doc = "\nA type tag for a value.\n\nTags are additional hints that a stream may use to interpret a value differently,\nor to avoid some unnecessary work.\n\nThe contents of a tag aren't considered public, only equality between two tag identifiers.\n"] # [derive (Clone , PartialEq , Eq)] pub struct Tag { id : u64 , data : & 'static str , }
    };
}

Tag!()