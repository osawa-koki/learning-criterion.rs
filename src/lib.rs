pub fn fibo(n: u64) -> u64 {
    if n < 2 {
        return n;
    }
    fibo(n - 1) + fibo(n - 2)
}
