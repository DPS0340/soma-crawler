fn parse_env(file_path: &str) {
    use std::fmt;
    use std::fs;

    let env_contents =
        fs::read_to_string(file_path).expect(format!("env file {} not exists", file_path).as_str());

    let mappings = env_contents
        .split("\n")
        .map(|line| {
            let splited: Vec<&str> = line.split("=").collect();

            match splited.len() {
                2 => splited,
                _ => panic!(format!(
                    ".env contents must follow key=val format, but {} given",
                    line
                )),
            }
        })
        .collect();
}

fn main() {
    println!("Hello, world!");
}
