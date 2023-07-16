use crate::error::Error;

pub fn read_password(prompt: &str) -> Result<String, Error> {
    Ok(dialoguer::Password::new().with_prompt(prompt).interact()?)
}

pub fn create_new_password() -> Result<String, Error> {
    let password = read_password("Create new password")?;
    let confirm_password = read_password("Confirm password")?;

    match password.eq(&confirm_password) {
        true => {
            println!("Password successfully set.");
            Ok(password)
        }
        false => {
            println!("Passwords do not match, please try again.");
            create_new_password()
        }
    }
}
