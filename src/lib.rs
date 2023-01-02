// Testing for a result type 
#[cfg(test)]
mod tests {
    #[test]
    fn get_sum() -> Result<(),String> {
        if 2+8 == 10 {
            Ok(())
        } else {
            Err(String::from("Invalid addition operation"))
        }
    }
}