use core::error;

fn main() {
    let file = "my.txt".to_owned();
    let data = read_file(&file);
    if let Some(data) = data {
        println!("{}", data);
    }
}

fn read_file(file_name: &str) -> Option<String> {
    std::fs::read_to_string(file_name)
        .ok()
        .and_then(|s| s.lines().next().map(|l| l.to_owned()))
}

fn parse_file(file_name: &str) -> Result<i32, Box<dyn error::Error>> {
    let file = std::fs::read_to_string(file_name)?;
    let i = file.parse()?;

    
    Ok(i)
}
