pub const SQL_INSERT_ACCOUNT: &str = "INSERT INTO user_accounts (
    created_at,
    nickname,
    email,
    password_salt,
    password_hash,
    account_encryption_key
) VALUES(?, ?, ?, ?, ?, ?)
RETURNING id";

pub const SQL_CHECK_EMAIL_EXISTS: &str = "SELECT 1
FROM user_accounts
WHERE email = ?";

pub const SQL_GET_ACCOUNT_HASH_INFO: &str = "SELECT id, password_salt
FROM user_accounts
WHERE email = ?";

pub const SQL_GET_ACCOUNT_ENCRYPTION_KEY: &str = "SELECT account_encryption_key
FROM user_accounts
WHERE id = ? AND password_hash = ?";

pub const SQL_INSERT_FILE: &str = "INSERT INTO files (
    public_id,
    owner_id,
    encryption_key,
    parent_folder_id,
    metadata,
    saved_location
) VALUES (?, ?, ?, ?, ?, ?)";
