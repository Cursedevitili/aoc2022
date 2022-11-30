pub mod shared;

#[cfg(test)]
mod tests {
    use std::io::Error;
    use crate::shared::load_input;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn read_file(){
        let contents = load_input("demo_input1.txt");
        match contents {
            Ok(text) => {println!("{}", text)}
            Err(e) => {println!("{}", e)}
        }
    }
}
