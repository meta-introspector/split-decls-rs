macro_rules! deps {
    () => {
        ForestObligation!();
        ObligationTreeId!();
        ObligationForest!();
    };
}

macro_rules! helper {
    () => {
        deps!();
        mod helper { use super :: * ; pub (super) type ObligationTreeIdGenerator = impl Iterator < Item = ObligationTreeId > ; impl < O : ForestObligation > ObligationForest < O > { # [define_opaque (ObligationTreeIdGenerator)] pub fn new () -> ObligationForest < O > { ObligationForest { nodes : vec ! [] , done_cache : Default :: default () , active_cache : Default :: default () , reused_node_vec : vec ! [] , obligation_tree_id_generator : (0 ..) . map (ObligationTreeId) , error_cache : Default :: default () , } } } }
    };
}

helper!();