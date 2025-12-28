macro_rules! Name {
    () => {
        # [doc = " A GraphQL name."] # [doc = ""] # [doc = " [Reference](https://spec.graphql.org/June2018/#Name)."] # [derive (Debug , Clone , PartialEq , Eq , PartialOrd , Ord , Hash)] pub struct Name (Arc < str >) ;
    };
}

Name!();