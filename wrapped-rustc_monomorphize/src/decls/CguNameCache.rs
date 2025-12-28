macro_rules! CguNameCache {
    () => {
        type CguNameCache = UnordMap < (DefId , bool) , Symbol > ;
    };
}

CguNameCache!()