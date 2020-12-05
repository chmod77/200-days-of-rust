// Scalar types represent a single value.
//Include integers, floating-point numbers, booleans and characters.
// Integer (Mutable and Immutable)

/**
 *
 * Integer types include:
 * 1. 8 bit => i8, u8
 * 2. 16 bit => i16, u16
 * 3. 32 bit => i32, u32
 * 4. 64 bit => i64, u64
 * 5. 128 bit => i128, u128
 * isize, usize => Useful when indexing collections
 *
 * DEFAULTS tp i32. It is the fastest
 */
pub fn run() {
    println!("Scalars mod called");

    let mut x = 10;
    println!("The value of x is {}", x);
    x = 6;
    println!("New value of x is {}", x);

    // FLoating Point types

    /**
     * These are numbers with decimal points
     * They are
     * 1. 32 bit => f32
     * 2. 64 bit => f64
     * Default type is f64 (Almost same speed as f32, but has more precision)
     *
     */
    let f32_example: f32 = 1.2;
    let f64_example: f64 = 1.3;
    let f64_default = 1.4;

    println!(
        "The floating point values are {}, {} and {}",
        f32_example, f64_example, f64_default
    );

    // Numeric Ops
    /**
     * Addition
     * Subtraction
     * Multiplication
     * Division
     * Remainder (Modulus)
     */
    // Addition
    let sum = 5 + 10;
    println!("The sum of 5 and 10 is {}", sum);

    // Subtraction
    let difference = 10 - 5;
    println!("The difference between 10 and 5 is {}", difference);

    //Division
    let division = 10 / 5;
    println!("The division of 10 by 5 is {}", division);

    // Product
    let product = 10 * 5;
    println!("The product of 10 and 5 is {}", product);

    // Remainder/Modulo
    let modulus = 10 % 5;
    println!("The remainder of 10 divided by 5 is {}", modulus);
    // MISCELLANEOUS

    let first: i64 = 3;
    let second: f64 = 45.2;

    let third = 0.000232;
    // Formatted printing on integers
    println!(
        "The numbers are {first}, {second} and {third}",
        first = first,
        second = second,
        third = third
    );

    // Maximum values
    let max_i64 = std::i64::MAX;
    let max_f64 = std::f64::MAX;
    let max_u128 = std::u128::MAX;
    let max_i32 = std::i32::MAX;
    println!("The maximum values for i64 is {max_i64}, f64 is {max_f64}, u128 is {max_u128} and i32 is {max_i32}",  max_i64=max_i64, max_u128=max_u128, max_f64=max_f64, max_i32=max_i32 );
}
