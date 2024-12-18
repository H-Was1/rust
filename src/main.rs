use std::io::Read;

fn main() {
    let file = "my.txt".to_owned();
    let data = read_file(&file).ok();
    if let Some(data) = data {
        println!("{}", data);
    }
}

fn read_file(file_name: &str) -> Result<String, std::io::Error> {
    let mut file = std::fs::File::open(file_name)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
