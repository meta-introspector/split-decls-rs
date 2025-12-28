macro_rules! check_unop {
    () => {
        # [doc = " While we currently allow all unary operations, we still want to explicitly guard against"] # [doc = " future changes here."] fn check_unop (op : mir :: UnOp) -> bool { use mir :: UnOp :: * ; match op { Not | Neg | PtrMetadata => true , } }
    };
}

check_unop!()