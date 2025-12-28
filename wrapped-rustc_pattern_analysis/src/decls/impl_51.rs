macro_rules! deps {
    () => {
        PatCx!();
        PatOrWild!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        impl < 'p , Cx : PatCx > Clone for PatOrWild < 'p , Cx > { fn clone (& self) -> Self { match self { PatOrWild :: Wild => PatOrWild :: Wild , PatOrWild :: Pat (pat) => PatOrWild :: Pat (pat) , } } }
    };
}

impl_51!()