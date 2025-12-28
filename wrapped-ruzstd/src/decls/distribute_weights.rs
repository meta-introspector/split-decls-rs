macro_rules! distribute_weights {
    () => {
        # [doc = " Distributes weights that add up to a clean power of two"] fn distribute_weights (amount : usize) -> Vec < usize > { assert ! (amount >= 2) ; assert ! (amount <= 256) ; let mut weights = Vec :: new () ; weights . push (1) ; weights . push (1) ; let mut target_weight = 1 ; let mut weight_counter = 2 ; while weights . len () < amount { let mut add_new = 1 << (weight_counter - target_weight) ; let available_space = amount - weights . len () ; if add_new > available_space { target_weight = weight_counter ; add_new = 1 ; } for _ in 0 .. add_new { weights . push (target_weight) ; } weight_counter += 1 ; } assert_eq ! (amount , weights . len ()) ; weights }
    };
}

distribute_weights!()