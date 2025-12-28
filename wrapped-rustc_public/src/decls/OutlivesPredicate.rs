macro_rules! OutlivesPredicate {
    () => {
        # [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub struct OutlivesPredicate < A , B > (pub A , pub B) ;
    };
}

OutlivesPredicate!();