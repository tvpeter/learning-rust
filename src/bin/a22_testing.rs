/*
Topic: Testing

Requirements:
Write tests for the existing program to ensure proper functionality

Notes:
- create at least two test cases for each function
- use 'cargo test' to test the program
- there are intiontional bugs in the program that need to be fiex
    - check the documentation comments for the functions to determine how they should operate
*/

///ensure n is >= lower and <= upper
fn clamp(n: i32, lower: i32, upper: i32) -> i32 {
    if n < lower {
        lower
    } else if n > upper {
        upper
    }else {
        n
    }
}

/// Divides a and b
fn div(a: i32, b: i32) -> Option<i32> {
    if b== 0 {
        return None;
    }
    
    Some(a/b)
}

/// takes two strings and places them immediately one after another
fn concat(first: &str, second: &str) -> String {
    format!("{} {}", first, second)
}



fn main(){}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn check_clamp(){
        let result = clamp(20, 30, 40);

        assert_eq!(result, 30, "should be 30");

        let upper_bound = clamp(150, 100, 120);
        assert_eq!(upper_bound, 120, "should be 120");
    }

    #[test]
    fn test_div(){
        let result = div(6, 2);

        assert_eq!(result, Some(3));

        let div_by_zero = div(2, 0);
        assert_eq!(div_by_zero, None, "cannot divide by zero");
    }

    #[test]
    fn check_concat(){
        let result = concat("hello", "world");

        assert_eq!(result, "hello world".to_owned());
    }
}
