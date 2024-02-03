// NOTE: 'if let' control flow
// The if let syntax lets you combine if and let into a less verbose way,
// to handle values that match one pattern while ignoring the rest. 

fn main() {
    
    // NOTE: What does 'Some' mean?
    
    // This means that the variable config_max could have a value, 
    // but could also not be present.

    // The variable type of config_max is a built-in enum called Option.
    // This Option enum can either be of one of two things:
    // Some(T) where T stands for a variable type
    // Or the enum can be None.

    let config_max: Option<u8> = Some(3);

    // So in this example, if the value of config_max exists,
    // then the value is 3 (which is a u8 aka unsigned 8bit integer)
    // If the value of config_max is not there, 
    // then it has the potential to be None
    // The Some() syntax is simply letting the compiler know that config_max,
    // may not always have a value

    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    /*
    This if statement is checking if "config_max"  has the Some wraper. 
    (E.g. Some(3))

    Then it looks for the specifc value in "config_max". In this case it is 3.
    Since those checks have been met, the value 3 is assigned to the variable max.

    And then println! prints out the value of max in the next line 
    
    */

    }

    // NOTE: When would you use `if let over the match expression`
    // Use `if let` when you are trying to target ONE specific condition,
    // and you don't care about the other outcomes.
    // =============================
    // Use `match` when you want something specific to happen,
    // for every possible outcome.

}
