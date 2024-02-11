fn main() { 
    let player_scores: [u32; 4] = [30, 50, 60, 20];
        
    for value in player_scores {
        println!("The score is {}", value);
    }
    
    // NOTE: Countdown 
    
    // (1..=5) means 1 to 5 (including 5)
    for value in (1..=5).rev() {
        println!("Fight begins in {} ...", value);
    }

    println!("Fight!!!!");

}
