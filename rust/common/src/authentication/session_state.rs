use actix_session::{Session, UserSession};
use actix_web::cookie::Display;
use actix_web::dev::Payload;
use actix_web::{Error, FromRequest, HttpRequest};
use serde::Deserialize;
use std::future::{ready, Ready};
use std::ops::Deref;
use std::sync::{Arc, Mutex};
use uuid::Uuid;
use wither::bson::oid::ObjectId;

// use actix_session::SessionExt;

#[derive(Deserialize)]
pub enum UserId {
    Uuid(Uuid),
    ObjectId(ObjectId),
}

impl Into<UserId> for ObjectId {
    fn into(self) -> UserId {
        UserId::ObjectId(self)
    }
}

impl Into<UserId> for Uuid {
    fn into(self) -> UserId {
        UserId::Uuid(self)
    }
}

// pub enum UserId {
//     Uuid,
//     ObjectId,
// }


pub struct TypedSession(pub Session);

impl TypedSession {
    const USER_ID_KEY: &'static str = "user_id";

    // pub fn new(session: Session) -> Self {
    //     let k = Arc::new(Mutex::new(session));
    //     Self(Arc::new(Mutex::new(session)))
    // }

    pub fn renew(&self) -> anyhow::Result<()> {
        self.0.renew();
        Ok(())
    }

    pub fn insert_user_id(&self, user_id: impl Into<UserId>) -> Result<(), Error> {
        let user_id: UserId = user_id.into();
        match user_id {
            UserId::Uuid(id) => self.0.insert(Self::USER_ID_KEY, id),
            UserId::ObjectId(id) => self.0.insert(Self::USER_ID_KEY, id),
        }
    }

    pub fn get_user_id(&self) -> Result<Option<impl Into<UserId>>, Error> {
        self.0.get::<UserId>(Self::USER_ID_KEY)
    }
}

/*
How will the request handlers build an instance of TypedSession?
We could provide a constructor that takes a Session as argument. Another option is to make TypedSession itself an actix-web extractor - let's try that out!
 */
impl FromRequest for TypedSession {
    // This is a complicated way of saying
    // "We return the same error returned by the
    // implementation of `FromRequest` for `Session`".
    type Error = <Session as FromRequest>::Error;
    // Rust does not yet support the `async` syntax in traits.
    // From request expects a `Future` as return type to allow for extractors
    // that need to perform asynchronous operations (e.g. a HTTP call)
    // We do not have a `Future`, because we don't perform any I/O,
    // so we wrap `TypedSession` into `Ready` to convert it into a `Future` that
    // resolves to the wrapped value the first time it's polled by the executor.
    type Future = Ready<Result<TypedSession, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        ready(Ok(TypedSession(req.get_session())))
    }
}

// #[derive(Clone, Copy, Debug)]
// pub struct UserId(Uuid);

// impl std::fmt::Display for UserId {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         self.0.fmt(f)
//     }
// }

// impl Deref for UserId {
//     type Target = Uuid;

//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

/*
How will the request handlers build an instance of TypedSession?
We could provide a constructor that takes a Session as argument. Another option is to make TypedSession itself an actix-web extractor - let's try that out!


*/
