pub const SQL_INSERT_ACCOUNT: &str = "INSERT INTO user_accounts (
    created_at,
    nickname,
    email,
    password_salt,
    password_hash,
    account_encryption_key
) VALUES(?, ?, ?, ?, ?, ?)";
