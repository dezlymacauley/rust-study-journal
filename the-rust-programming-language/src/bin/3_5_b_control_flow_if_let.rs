// NOTE: 'If let' 
// This will assign a value to a variable,
// depending on whether a condition is true or false

fn main() {
    
    let team_has_healer: bool = true;

    let survival_rate: i32 = if team_has_healer {70} else {40};
    // If the team has a healer, then the survival_rate is 70,
    // if the team does not have a healer, then the survival rate is 40

    println!("Your team's chances of surving are {} percent.", survival_rate);

}
