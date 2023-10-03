use std::future::{ready, Ready};

use actix_session::{Session, SessionExt, SessionGetError, SessionInsertError};
use actix_web::{dev::Payload, FromRequest, HttpRequest};
use uuid::Uuid;

pub struct TypedSession(Session);

impl FromRequest for TypedSession {
    // A complicated way of saying "we return the same error as the error
    // returned by the implementation of `FromRequest` for `Session`"
    type Error = <Session as FromRequest>::Error;
    // Since rust does not yet support the `async` syntax in traits and
    // FromRequest expects a `Future` as return type to allow for extractors
    // that need to perform asynchronous operation (e.g. a HTTP call)
    // We don't have a `Future`, because we don't perform an I/O
    // so we wrap `TypedSession` into `Ready` to convert it into a `Future`
    // that resolves the first time it's polled by the executor.
    type Future = Ready<Result<TypedSession, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        ready(Ok(TypedSession(req.get_session())))
    }
}

impl TypedSession {
    const USER_ID_KEY: &'static str = "user_id";

    pub fn renew(&self) {
        self.0.renew();
    }

    pub fn insert_user_id(&self, user_id: Uuid) -> Result<(), SessionInsertError> {
        self.0.insert(Self::USER_ID_KEY, user_id)
    }

    pub fn get_user_id(&self) -> Result<Option<Uuid>, SessionGetError> {
        self.0.get(Self::USER_ID_KEY)
    }
}
