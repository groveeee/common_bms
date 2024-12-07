use chrono::{Duration, Local};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use log::info;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// jwt工具类

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    // pub aud: String,
    pub userid: String,// 用户ID
    // pub company: String,
    pub exp: u64,// 过期时间
}
impl Claims {
    pub fn new(sub: String, exp: u64) -> Self {
        Self { userid: sub, exp }
    }
}

pub fn create_token(uuid:Uuid)->String{
    let now = Local::now();
    let expire = (now + Duration::minutes(60)).timestamp();
    println!("{}", expire);
    let claims = Claims {
        userid: uuid.to_string(),
        exp: expire as u64,
    };
    let  token = encode(&Header::default(), &claims, &EncodingKey::from_secret("secret".as_bytes())).unwrap();
    token
}

pub fn verify_token(token:String)->Result<Claims,String>{
    let mut validation = Validation::new(Algorithm::HS256);
    validation.set_required_spec_claims(&["exp", "userid"]);
    // 在令牌之前的一段时间（以秒为单位）拒绝令牌， exp 以防止在通过网络传输时过期
    validation.reject_tokens_expiring_in_less_than = 60;
    // 默认就会校验过期时间
    validation.validate_exp = true;
    let token_data = match decode::<Claims>(&token, &DecodingKey::from_secret("secret".as_ref()), &validation)
     {
        Ok(c) => {c}
         Err(err) => {
             let kind = err.into_kind();
             info!("{:?}", kind);
             return Err("err".to_string());
         }
    };
    Ok(token_data.claims)
}