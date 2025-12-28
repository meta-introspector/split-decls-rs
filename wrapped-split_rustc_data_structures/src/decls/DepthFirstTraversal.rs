macro_rules! deps {
    () => {
        Direction!();
        LinkedGraph!();
        NodeIndex!();
    };
}

macro_rules! DepthFirstTraversal {
    () => {
        deps!();
        pub struct DepthFirstTraversal < 'g , N , E > { graph : & 'g LinkedGraph < N , E > , stack : Vec < NodeIndex > , visited : DenseBitSet < usize > , direction : Direction , }
    };
}

DepthFirstTraversal!();