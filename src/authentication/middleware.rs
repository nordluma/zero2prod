use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    error::InternalError,
    FromRequest,
};
use actix_web_lab::middleware::Next;

use crate::{
    session_state::TypedSession,
    utils::{e500, see_other},
};

pub async fn reject_anonymous_users(
    mut req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, actix_web::Error> {
    let session = {
        let (http_request, payload) = req.parts_mut();
        TypedSession::from_request(http_request, payload).await
    }?;

    match session.get_user_id().map_err(e500)? {
        Some(_) => next.call(req).await,
        None => {
            let response = see_other("/login");
            let e = anyhow::anyhow!("The user has not logged in");

            Err(InternalError::from_response(e, response).into())
        }
    }
}
