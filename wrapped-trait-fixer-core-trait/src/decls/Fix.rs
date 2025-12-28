macro_rules! Fix {
    () => {
        # [derive (Debug)] pub enum Fix < S , D , ID > { AddDerive { span : S , trait_name : String } , AddCloneImpl { def_id : D } , RemoveImpl { item_id : ID } , }
    };
}

Fix!()