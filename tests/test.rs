
extern crate domi;
#[cfg(test)]
mod tests {
    use domi::parser;
    use domi::hello;

    #[test]
    fn module_hello_test() {
        assert_eq!(hello::hi(), "Hello\r\n");
    }

    #[test]
    fn module_path_test() {
        let data_written = parser::create_file_with_data(String::from("test.toml"), hello::hi());
        assert_eq!(data_written.unwrap(), hello::hi().into_bytes().len());

    }
    #[test]
    fn module_read_file_test() {
        let filename = String::from("test.toml");
        let file_content = parser::read_file(&filename);
        assert_eq!(hello::hi(), file_content);
    }
}
