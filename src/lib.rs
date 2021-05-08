struct Kyakutanka(i64);
struct Kyakusuu(u64);

#[derive(Debug,PartialEq)]
struct Uriage(i64);

#[derive(Debug,PartialEq)]
struct Hiyo(i64);

impl Uriage {
    pub fn new(tanka: Kyakutanka, kazu: Kyakusuu) -> Uriage {
        Uriage(tanka.0 * kazu.0 as i64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_test() {
        let a = 1000;
        let b = 100;
        let multiple = a * b;
        let a = Kyakutanka(1000);
        let b = Kyakusuu(100);
        assert_eq!(Uriage::new(a, b), Uriage(multiple));
    }
}
