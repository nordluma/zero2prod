use actix_web::{http::header::ContentType, web, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct QueryParams {
    error: Option<String>,
}

pub async fn login_form(query: web::Query<QueryParams>) -> HttpResponse {
    let html_error = match query.0.error {
        Some(error_message) => {
            format!("<p><i>{}</i></p>", error_message)
        }
        None => "".into(),
    };

    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#"<!doctype html>
            <html lang="en">
              <head>
                <meta 
                    http-equiv="content-type"
                    content="text/html"
                    charset="utf-8"
                />
                <title>Login</title>
              </head>
              <body>
                {}
                <form action="/login" method="post">
                  <label
                    >Username
                    <input 
                        type="text"
                        placeholder="Enter Username"
                        name="username"
                    />
                  </label>
                  <label
                    >Password
                    <input
                        type="password"
                        placeholder="Enter Password"
                        name="password"
                    />
                  </label>
                  <button type="submit">Login</button>
                </form>
              </body>
            </html>"#,
            html_error
        ))
}
