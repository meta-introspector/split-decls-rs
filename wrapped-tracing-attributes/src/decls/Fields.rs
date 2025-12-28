macro_rules! deps {
    () => {
        Field!();
    };
}

macro_rules! Fields {
    () => {
        deps!();
        # [derive (Clone , Debug)] pub (crate) struct Fields (pub (crate) Punctuated < Field , Token ! [,] >) ;
    };
}

Fields!()