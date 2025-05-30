#[allow(dead_code)]
pub fn factorial(n: u64) -> u64 {
    match n {
        0 => 1,
        _ => n * factorial(n - 1),
    }
}

pub fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIB_OUTPUT: [u64; 7] = [0, 1, 1, 2, 3, 5, 8];
    const FAC_OUTPUT: [u64; 6] = [1, 1, 2, 6, 24, 120];

    #[test]
    fn test_fib() {
        for i in 0..6 {
            assert_eq!(fibonacci(i), FIB_OUTPUT[i as usize]);
        }
    }

    #[test]
    fn test_fac() {
        for i in 0..5 {
            assert_eq!(factorial(i), FAC_OUTPUT[i as usize]);
        }
    }
}
