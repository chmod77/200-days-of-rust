/**
 * Functions use snake_case conventions
 *
 * Functions take parameters
 * 
 * Functions can take concrete parameters => called arguments
 * 
 * All param types must be declared.
 * 
 * Multiple params are separated by commas.
 * 
 * Function body is made up of statements and expressions
 * 
 * STATEMENTS do not evaluate to something. THey end in as semi-colon
 * eg. let y = 6;
 * 
 * EXPRESSIONS evaluate to something. They do not end in a semi-colon.
 * eg 5+6 is an expression that evaluates to 11.
 * 
 * Examples of expressions:
 * 1. Calling a function
 * 
 * 2. Calling a macro
 * 
 * 3. Block used to create new scopes {}
 * 
 * 
 */

pub fn run(x: i32) {
    println!("The parameter is {}", x);
}


pub fn explore(){
    // Statements

    let y = 6;

    // Expressions
    let expr = {
        let x = 3;
        x + 2
    };

    println!("{}", expr);
}