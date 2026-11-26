pub fn answer() -> u32 {
    let mut total = 0;
    for i in 0..10 {
        total += i;
    }
    total
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(super::answer(), 45);
    }
}
