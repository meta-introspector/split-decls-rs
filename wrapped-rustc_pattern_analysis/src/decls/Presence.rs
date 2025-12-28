macro_rules! Presence {
    () => {
        # [doc = " Whether we have seen a constructor in the column or not."] # [derive (Debug , Clone , Copy , PartialEq , Eq , PartialOrd , Ord)] enum Presence { Unseen , Seen , }
    };
}

Presence!()