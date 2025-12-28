macro_rules! deps {
    () => {
        EditionedFileId!();
    };
}

macro_rules! _ {
    () => {
        deps!();
        const _ : () = assert ! (EditionedFileId :: RESERVED_MASK ^ EditionedFileId :: EDITION_MASK ^ EditionedFileId :: FILE_ID_MASK == 0xFFFF_FFFF) ;
    };
}

_!();