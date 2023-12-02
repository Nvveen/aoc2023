pub fn read_file(filename: &str) -> String {
    use std::fs::File;
    use std::io::prelude::*;
    let mut file = File::open(filename).expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    contents
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(read_file("src/test.txt"), "test");
    }
}
