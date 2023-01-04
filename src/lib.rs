// Testing for a result type 
#[cfg(test)]
mod tests {
    #[test]
    fn get_sum1() -> Result<(),String> {
        if 2+8 == 10 {
            Ok(())
        } else {
            Err(String::from("Invalid addition operation"))
        }
    }
}

#[test]
    fn get_sum2() {
        assert_eq!(2+2,4)
    }