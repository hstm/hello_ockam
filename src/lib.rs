mod echoer;
pub use echoer::*;

mod hop;
pub use hop::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
        druck();
    }

    fn druck() {
        let name = "Bob".to_string();
        println!("Hello {}", name);
    }
}
