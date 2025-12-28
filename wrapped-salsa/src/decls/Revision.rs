macro_rules! Revision {
    () => {
        # [doc = " A unique identifier for the current version of the database."] # [doc = ""] # [doc = " Each time an input is changed, the revision number is incremented."] # [doc = " `Revision` is used internally to track which values may need to be"] # [doc = " recomputed, but is not something you should have to interact with"] # [doc = " directly as a user of salsa."] # [derive (Copy , Clone , PartialEq , Eq , Hash , PartialOrd , Ord)] pub struct Revision { generation : NonZeroUsize , }
    };
}

Revision!()