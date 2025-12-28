macro_rules! Review {
    () => {
        # [derive (Debug , Serialize)] pub struct Review { title : String , paragraphs : Vec < String > , }
    };
}

Review!();