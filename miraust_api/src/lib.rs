pub mod bot;
pub mod contact;
pub mod plugin;
pub mod event;
pub(crate) mod managed;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn length() {
        println!("{}", 1);
    }
}
