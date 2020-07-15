mod components;
use components::*;

use epoch_timestamp::Epoch;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};

pub struct JWT<T> {
    key: String,
    algorithm: Algorithm,
    exp: u64,
    phantom: std::marker::PhantomData<T>,
}

impl<T> JWT<T> {
    pub fn new(key: String, algorithm: String, expire: String) -> Result<JWT<T>, JWTError> {
        let algorithm = match algorithm.as_ref() {
            "HS256" => Algorithm::HS256,
            "HS384" => Algorithm::HS384,
            "HS512" => Algorithm::HS512,
            "ES256" => Algorithm::ES256,
            "ES384" => Algorithm::ES384,
            "RS256" => Algorithm::RS256,
            "RS384" => Algorithm::RS384,
            "RS512" => Algorithm::RS512,
            "PS256" => Algorithm::PS256,
            "PS384" => Algorithm::PS384,
            "PS512" => Algorithm::PS512,
            _ => return Err(JWTError::new("".to_string())),
        };

        let mut chars: std::vec::Vec<char> = expire.chars().collect();
        let last_character = chars.pop().expect("empty expire string");
        let number: u64 = String::from_iter(chars.into_iter())
            .parse()
            .expect("invalid number value");

        let exp = match last_character {
            's' => Epoch::second(number),
            'm' => Epoch::minute(number),
            'h' => Epoch::hour(number),
            'd' => Epoch::day(number),
            'w' => Epoch::week(number),
            'y' => Epoch::year(number),
            _ => return Err(JWTError::new("".to_string())), //invalid suffix
        };

        Ok(JWT {
            key,
            algorithm,
            exp,
            phantom: std::marker::PhantomData,
        })
    }

    pub fn sign(data: T) -> Result<String, JWTError> {
        let token_result = jsonwebtoken::encode(
            &Header::new(Algorithm::HS256),
            &payload,
            &EncodingKey::from_secret(self.key),
        );

        if (token_result.is_ok()) {
            Ok(token_result.unwrap().to_string())
        } else {
            Err(JWTError::new("".to_string()))
        }
    }

    pub fn verify(token: String) -> Result<T, JWTError> {
        let decoded_result = jsonwebtoken::decode::<Payload>(
            &token,
            &DecodingKey::from_secret(self.key),
            &Validation::new(Algorithm::HS256),
        );

        if (decoded_result.is_ok()) {
            Ok(decoded_result.unwrap().claims.data)
        } else {
            Err(JWTError::new("invalid token".to_string()))
        }
    }
}
