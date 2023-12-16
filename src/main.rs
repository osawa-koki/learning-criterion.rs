use mylib::fibo;

fn main() {
    for i in 0..10 {
        println!("{}: {}", i, fibo(i));
    }
}
