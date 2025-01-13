mod utils {
    use std::{fs, io};
    use std::path::Path;
    use std::io::Result;

    pub fn read_file(filename: &str) -> Result<String> {
        return fs::read_to_string(filename);
    }

    fn check_file_exists(path: &str) -> bool {
        let filepath: &Path = Path::new(path);
        let exists: bool = Path::exists(filepath);

        return exists;
    }
}

#[cfg(test)]
mod utils_tests {
    use crate::utils;
    //use std::{fmt::Result, result};
    use std::{fmt::Result, io::Result};

    #[test]
    fn read_file_no_file_not_found_should_return_error() {
        let file: Result<String> = crate::utils::utils::read_file("nofile.txt");

        let mut result: String = String::new();
        match file {
            Ok(value) => result = value,
            Err(err) => result = format!("{err}")
        };

        assert_eq!(result, "The system cannot find the path specified. (os error 3)");
    }


//    #[test]
//    fn read_file_file_found_should_return_contents() {
//        let file: Result<String> = crate::utils::utils::read_file(".\\Cargo.toml");
//
//        let mut result: String = String::new();
//        match file {
//            Ok(value) => result = value.lines()[0],
//            Err(err) => result = format!("{err}")
//        };
//
//        assert_eq!(result, "[package]");
//    }
}
