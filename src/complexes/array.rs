/**
 * Arrays
 * 
 * All elements must be of same type
 * 
 * Have fixed length
 * 
 * Comma separated, between square brackets
 * 
 * Accesed via indexing, i.e a[0]
 */
pub fn run(){
    println!("Arrays mod called");
    let a = [1,2,3,4];
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August"
    ];
    // Typed array => 
    let even: [i32; 4] = [2,4,6,8];

    // Initialize an array with same values => 6 elements in array, with value 3 i.e [3,3,3,3,3,3,3]
    let same = [3;6];

    let first = same[0];
    let second = same[1];

    println!("{}, {}", first, second);
}