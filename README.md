## poem_openapi的validator集成

## 例子
register.rs
```rust
use validator::Validate;
use poem_openapi::Object;
use poem_openapi_validator_derive::Validation;

#[derive(Object, Validate, Validation)]
pub struct Register {
    #[validate(length(min = 6, max = 20, message = "user field length error"))]
    pub username: String,
    #[validate(length(min = 6, max = 20, message = "password field length error"))]
    pub password: String,
}
```

auth.rs
```rust
use poem_openapi::{payload::PlainText, OpenApi};
use poem_openapi::payload::Json;

#[OpenApi]
impl Authorize {
    #[oai(path="/register", method="post")]
    async fn register(&self, register: Json<Register>) -> PlainText<String> {
        //validator request
        match register.validator_is_error() {
            Some(error) => PlainText(error),
            None => PlainText("ok".to_string())
        }
    }
}
```