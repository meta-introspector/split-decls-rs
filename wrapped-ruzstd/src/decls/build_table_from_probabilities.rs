macro_rules! deps {
    () => {
        State!();
        FSETable!();
        SymbolStates!();
    };
}

macro_rules! build_table_from_probabilities {
    () => {
        deps!();
        pub (super) fn build_table_from_probabilities (probs : & [i32] , acc_log : u8) -> FSETable { let mut states = core :: array :: from_fn :: < SymbolStates , 256 , _ > (| _ | SymbolStates { states : Vec :: new () , probability : 0 , }) ; let mut negative_idx = (1 << acc_log) - 1 ; for (symbol , _prob) in probs . iter () . copied () . enumerate () . filter (| prob | prob . 1 == - 1) { states [symbol] . states . push (State { num_bits : acc_log , baseline : 0 , last_index : (1 << acc_log) - 1 , index : negative_idx , }) ; states [symbol] . probability = - 1 ; negative_idx -= 1 ; } let mut idx = 0 ; for (symbol , prob) in probs . iter () . copied () . enumerate () { if prob <= 0 { continue ; } states [symbol] . probability = prob ; let states = & mut states [symbol] . states ; for _ in 0 .. prob { states . push (State { num_bits : 0 , baseline : 0 , last_index : 0 , index : idx , }) ; idx = next_position (idx , 1 << acc_log) ; while idx > negative_idx { idx = next_position (idx , 1 << acc_log) ; } } assert_eq ! (states . len () , prob as usize) ; } for (symbol , prob) in probs . iter () . copied () . enumerate () { if prob <= 0 { continue ; } let prob = prob as u32 ; let state = & mut states [symbol] ; state . states . sort_by (| l , r | l . index . cmp (& r . index)) ; let prob_log = if prob . is_power_of_two () { prob . ilog2 () } else { prob . ilog2 () + 1 } ; let rounded_up = 1u32 << prob_log ; let double_states = rounded_up - prob ; let single_states = prob - double_states ; let num_bits = acc_log - prob_log as u8 ; let mut baseline = (single_states as usize * (1 << (num_bits))) % (1 << acc_log) ; for (idx , state) in state . states . iter_mut () . enumerate () { if (idx as u32) < double_states { let num_bits = num_bits + 1 ; state . baseline = baseline ; state . num_bits = num_bits ; state . last_index = baseline + ((1 << num_bits) - 1) ; baseline += 1 << num_bits ; baseline %= 1 << acc_log ; } else { state . baseline = baseline ; state . num_bits = num_bits ; state . last_index = baseline + ((1 << num_bits) - 1) ; baseline += 1 << num_bits ; } } state . states . sort_by (| l , r | l . baseline . cmp (& r . baseline)) ; } FSETable { table_size : 1 << acc_log , states , } }
    };
}

build_table_from_probabilities!()