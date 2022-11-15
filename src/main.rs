fn main() {
    //find the modular multiplicative inverse of (3, 11)
    let a = 3_i64;  // 3 represented in 64bits integer
    let m = 11_i64; // 11 represented in 64bits integer

    //function call 
    mod_inverse(a, m);
}
//Function that calculates modular muliplicative inverse using Euclidean algorithm
fn mod_inverse(a: i64, m: i64) {
    let mut x: i64 = 0;      // Initialize multable valuable x value in 64 bit
    let mut y: i64 = 0;     // Initialize multable  valuable y value in 64 bit

    let g = gcd_extended(a, m, &mut x, &mut y);  //parse the value of a, m, x, y into the extended euclidean function
    if g != 1 {
        println!("Inverse does not exist");
    } else {
        // M is added to handle negative x
        let result = (x % m + m ) % m;
        println!("The modular multiplicative inverse is: {}", result);
    }
}
    
 
// Function for extended Euclidean Algorithm
 fn  gcd_extended(a: i64, b: i64, x: &mut i64, y: &mut i64) -> i64{

    // Base Case
    if a == 0 {
        *x = 0;
        *y = 1;
        return b;
    }
      
    let mut x1 =  0;    // Initialize multable variable x1 value in 64 bit
    let mut y1 =  0;    // Initialize multable variable y1 value in 64 bit

    // To store results of recursive call in gcd variable 
    let gcd = gcd_extended(b % a, a, &mut x1,  &mut y1);

 
    // Update x and y using results of recursive
    // call 
    let temp = b /a;
    *x = y1 - temp * x1;
    *y = x1;
  
    return gcd;
}
