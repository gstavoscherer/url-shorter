use crate::error::AppError;

const ALPHABET: &[char] = &[
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm',
    'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
    'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
];

pub fn generate(length: usize) -> String {
    nanoid::nanoid!(length, ALPHABET)
}

pub fn validate_alias(alias: &str) -> Result<(), AppError> {
    if alias.len() < 3 || alias.len() > 32 {
        return Err(AppError::BadRequest(
            "Custom alias must be between 3 and 32 characters".to_string(),
        ));
    }

    if !alias
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '-')
    {
        return Err(AppError::BadRequest(
            "Custom alias must contain only alphanumeric characters and hyphens".to_string(),
        ));
    }

    if alias.starts_with('-') || alias.ends_with('-') {
        return Err(AppError::BadRequest(
            "Custom alias must not start or end with a hyphen".to_string(),
        ));
    }

    Ok(())
}
