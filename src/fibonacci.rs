// Computes the nth fibonacci number
pub fn fibonacci(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }
    
    let mut first = 0;
    let mut second = 1;
    let mut third = first + second;

    for _nth_term in 2..n + 1 {
        third = first + second;
        first = second;
        second = third;
    }

    return third;

}