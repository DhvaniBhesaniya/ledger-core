#![allow(dead_code)]
pub fn validate_amount(amount: i64) -> Result<(), String> {
    if amount <= 0 {
        return Err("Amount must be positive".to_string());
    }
    Ok(())
}

pub fn validate_email(email: &str) -> Result<(), String> {
    if email.contains('@') && email.len() > 5 {
        Ok(())
    } else {
        Err("Invalid email".to_string())
    }
}
