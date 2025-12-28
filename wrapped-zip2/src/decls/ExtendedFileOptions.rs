macro_rules! ExtendedFileOptions {
    () => {
        # [doc = " The Extension for Extra Data and Central Extra Data"] # [derive (Clone , Default , Eq , PartialEq)] pub struct ExtendedFileOptions { extra_data : Arc < Vec < u8 > > , central_extra_data : Arc < Vec < u8 > > , }
    };
}

ExtendedFileOptions!()