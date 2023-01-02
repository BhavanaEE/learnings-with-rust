pub fn greeting(name: &str) -> String {
    format!("Hello! {}", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let greet = greeting("Bhavana");
        
        // assert with custom message
        assert!(greet.contains("na"),"Greeting doesn't contain name, value was {}",greet); 
    }
}