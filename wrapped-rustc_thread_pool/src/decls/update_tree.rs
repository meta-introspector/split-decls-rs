macro_rules! deps {
    () => {
        Tree!();
    };
}

macro_rules! update_tree {
    () => {
        deps!();
        # [test] fn update_tree () { let mut tree : Tree < u32 > = random_tree (10) ; let values : Vec < u32 > = tree . iter () . cloned () . collect () ; tree . update (| v | * v += 1) ; let new_values : Vec < u32 > = tree . iter () . cloned () . collect () ; assert_eq ! (values . len () , new_values . len ()) ; for (& i , & j) in values . iter () . zip (& new_values) { assert_eq ! (i + 1 , j) ; } }
    };
}

update_tree!();