macro_rules! deps {
    () => {
        Config!();
        Active!();
        Action!();
        Slab!();
    };
}

macro_rules! run {
    () => {
        deps!();
        fn run < C : Config > (actions : Vec < Action >) -> Result < () , TestCaseError > { let mut slab = Arc :: new (Slab :: new_with_config :: < C > ()) ; let mut active = Active :: default () ; for action in actions { tid :: with (action . tid , | | { apply_action :: < C > (& slab , & mut active , action . kind) }) ? ; } let mut expected_values = Vec :: new () ; for (key , value) in active . drain () { prop_assert ! (slab . contains (key)) ; prop_assert_eq ! (slab . get (key) . map (| e | * e) , Some (value)) ; prop_assert_eq ! (slab . clone () . get_owned (key) . map (| e | * e) , Some (value)) ; expected_values . push (value) ; } expected_values . sort () ; let slab = Arc :: get_mut (& mut slab) . unwrap () ; let mut actual_values = slab . unique_iter () . copied () . collect :: < Vec < _ > > () ; actual_values . sort () ; prop_assert_eq ! (actual_values , expected_values) ; Ok (()) }
    };
}

run!();