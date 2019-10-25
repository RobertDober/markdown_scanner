#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn my_test() {
        assert_eq!(vsn(), "0.0.1")
    }
}
pub fn vsn() -> String {
    String::from("0.0.1")
}
