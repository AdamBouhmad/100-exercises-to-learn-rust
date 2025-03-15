// Rewrite the factorial function using a `for` loop.
pub fn factorial(mut n: u32) -> u32 {

    let mut num: u32 = 1; 

 /*   let mut sum = 0;
    for i in 1..=5 {
        sum += i;
}
*/
    for i in 1..=n{
        num *=n;
        println!("{num}");

        n -= 1; 
    }

    num

}


/*let mut num: u32 = 1;  // Start with 1 since 0! = 1
    
while n > 0 {
    num *= n;  // Multiply num by n
    n -= 1;    // Decrease n
}

num  // Return the computed factorial
}
*/



#[cfg(test)]
mod tests {
    use crate::factorial;

    #[test]
    fn first() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn second() {
        assert_eq!(factorial(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn fifth() {
        assert_eq!(factorial(5), 120);
    }
}
