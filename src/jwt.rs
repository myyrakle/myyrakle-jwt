use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct JWT<T> {
    key: String,
    algorithm: String,
    expire: String,
}

impl<T> JWT<T> {
    pub fn new(key: String, algorithm: String, expire: String) -> Result<JWT<T>, JWTError> {
        Ok(JWT {
            key,
            algorithm,
            expire,
        })
    }

    pub fn sign() -> Result<JWT<T>, JWTError> {
        let token = jsonwebtoken::encode(
            &Header::new(Algorithm::HS256),
            &Payload::new(),
            &EncodingKey::from_secret(key),
        )
        .unwrap();
    }

    pub fn verify() -> Result<T, JWTError> {
        let key = "1jhficnew243213vdv".as_ref();

        println!("{}", token);
        jsonwebtoken::decode::<Payload>(
            &token,
            &DecodingKey::from_secret(key),
            &Validation::new(Algorithm::HS256),
        )
        .expect("토큰 검증 실패");
    }
}
