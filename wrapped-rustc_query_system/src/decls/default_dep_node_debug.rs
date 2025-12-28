macro_rules! deps {
    () => {
        DepNode!();
    };
}

macro_rules! default_dep_node_debug {
    () => {
        deps!();
        pub fn default_dep_node_debug (node : DepNode , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("DepNode") . field ("kind" , & node . kind) . field ("hash" , & node . hash) . finish () }
    };
}

default_dep_node_debug!();