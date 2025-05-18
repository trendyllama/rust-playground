

pub fn factorial(n: u64) -> u64 {

    return match n {
        0 => 1,
        _ => n * factorial(n - 1)
    }

}


pub fn fibonacci(n: u64) -> u64 {

    return match n {

        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2)


    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib() {

        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(2), 1);
        assert_eq!(fibonacci(3), 2);
        assert_eq!(fibonacci(4), 3);
        assert_eq!(fibonacci(5), 5);
        assert_eq!(fibonacci(6), 8);


    }

    #[test]
    fn test_fac() {

        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(5), 120);
    }
}
