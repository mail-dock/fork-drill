fn fib(n: u32) -> u64 {
    if n < 2 { n as u64 } else { fib(n - 1) + fib(n - 2) }
}

fn main() { println!("{}", fib(20)); }

#[cfg(test)]
mod tests {
    use super::fib;
    #[test]
    fn twenty() { assert_eq!(fib(20), 6765); }
}
