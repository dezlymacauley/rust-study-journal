// NOTE: Hash Map

// In order to use a Hash Map you need to add this line.
use std::collections::HashMap;

fn main () {

    
    // NOTE: Step 1 - Create the Hashmap
    let mut scores: HashMap<String, i32> = HashMap::new();

    // NOTE: Step 2 - Create the keys for the HashMap
    let blue_team: String = String::from("Blue Team");
    let yellow_team: String = String::from("Yellow Team");

    // NOTE: Step 3 - Add values to the keys
    // The syntax is:
    // name-of-hashmap.insert(key, value);
    
    scores.insert(blue_team, 10);    
    scores.insert(yellow_team, 10);    
    // NOTE: This will actually move the ownership of the strings,
    // into the hasmap!
    // If you try to print the variables blue_team and yellow_team,
    // from Step 2, this will not work because they have been moved.
    

}
