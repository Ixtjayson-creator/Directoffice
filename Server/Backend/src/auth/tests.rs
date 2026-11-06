#[cfg(test)]
mod tests {
    use crate::auth::{hash_password, verify_password, generate_token, verify_token};
    use uuid::Uuid;

    #[test]
    fn test_password_hash_and_verify() {
        let password = "SuperSecretPassword123!";
        let hash = hash_password(password).expect("hashing should succeed");
        assert!(verify_password(password, &hash));
        assert!(!verify_password("WrongPassword", &hash));
    }

    #[test]
    fn test_jwt_generation_and_verification() {
        let user_id = Uuid::new_v4();
        let org_id = Uuid::new_v4();
        let role = "admin";
        let secret = b"my_test_secret_key_123456789";

        let token = generate_token(user_id, org_id, role, secret).expect("token creation should succeed");
        let claims = verify_token(&token, secret).expect("token verification should succeed");

        assert_eq!(claims.sub, user_id);
        assert_eq!(claims.org, org_id);
        assert_eq!(claims.role, role);
    }
}
