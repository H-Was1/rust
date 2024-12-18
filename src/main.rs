fn main() {
    let user_name = String::from("user");
}

fn get_user_id(user_name: &str) -> Result<u32, String> {
    if user_name.is_empty() {
        return Err("User name is empty".to_string());
    }

    Ok(20)
}
