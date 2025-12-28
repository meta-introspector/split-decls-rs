macro_rules! deps {
    () => {
        FSETable!();
    };
}

macro_rules! build_table_from_counts {
    () => {
        deps!();
        fn build_table_from_counts (counts : & [usize] , max_log : u8 , avoid_0_numbit : bool) -> FSETable { let mut probs = [0 ; 256] ; let probs = & mut probs [.. counts . len ()] ; let mut min_count = 0 ; for (idx , count) in counts . iter () . copied () . enumerate () { probs [idx] = count as i32 ; if count > 0 && (count < min_count || min_count == 0) { min_count = count ; } } min_count -= 1 ; let mut max_prob = 0i32 ; for prob in probs . iter_mut () { if * prob > 0 { * prob -= min_count as i32 ; } max_prob = max_prob . max (* prob) ; } if max_prob > 0 && max_prob as usize > probs . len () { let divisor = max_prob / (probs . len () as i32) ; for prob in probs . iter_mut () { if * prob > 0 { * prob = (* prob / divisor) . max (1) } } } let sum = probs . iter () . sum :: < i32 > () ; assert ! (sum > 0) ; let sum = sum as usize ; let acc_log = (sum . ilog2 () as u8 + 1) . max (5) ; let acc_log = u8 :: min (acc_log , max_log) ; if sum < 1 << acc_log { let diff = (1 << acc_log) - sum ; let max = probs . iter_mut () . max () . unwrap () ; * max += diff as i32 ; } else { let mut diff = sum - (1 << acc_log) ; while diff > 0 { let min = probs . iter_mut () . filter (| prob | * * prob > 1) . min () . unwrap () ; let decrease = usize :: min (* min as usize - 1 , diff) ; diff -= decrease ; * min -= decrease as i32 ; } } let max = probs . iter_mut () . max () . unwrap () ; if avoid_0_numbit && * max > 1 << (acc_log - 1) { let redistribute = * max - (1 << (acc_log - 1)) ; * max -= redistribute ; let max = * max ; let second_max = * probs . iter_mut () . filter (| x | * * x != max) . max () . unwrap () ; let second_max = probs . iter_mut () . find (| x | * * x == second_max) . unwrap () ; * second_max += redistribute ; assert ! (* second_max <= max) ; } build_table_from_probabilities (probs , acc_log) }
    };
}

build_table_from_counts!();