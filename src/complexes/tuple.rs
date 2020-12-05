// Tuples;
/**
 * Tuples allow different types of components
 *
 * Have fixed length. Once declared, cannot be adjusted
 *
 * Values are enterd into a comma-separated list inside parentheses.
 *
 * Values in a tuple can be destructured.
 * 
 * Values in a tuple can be accessed via dot notation, i.e 
 * 
 * tup.0 to access first element on tuple, tup.1 to access second etc etc
 *
 */
pub fn run() {
    println!("Called tuples");
    let tup: (i64, f64, u8) = (500, 300.3, 1);
    let(first, second, third) = tup;
    println!("{}, {}, {}", tup.0, tup.1, tup.2);
    println!("{}, {}, {}", first, second, third);
}
