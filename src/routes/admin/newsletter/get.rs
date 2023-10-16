use std::fmt::Write;

use actix_web::{http::header::ContentType, HttpResponse};
use actix_web_flash_messages::IncomingFlashMessages;

pub async fn publish_newsletter_form(
    flash_messages: IncomingFlashMessages,
) -> Result<HttpResponse, actix_web::Error> {
    let mut html_msg = String::new();

    for m in flash_messages.iter() {
        writeln!(html_msg, "<p><i>{}</i></p>", m.content()).unwrap();
    }

    let idempotency_key = uuid::Uuid::new_v4();

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#"
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta http-equiv="content-type" content="text/html; charset=utf-8">
                <title>Publish Newsletter Issue</title>
            </head>
            <body>
                {}
                <form action="/admin/newsletters" method="post">
                    <label>Title:<br>
                        <input
                            type="text"
                            placeholder="Enter issue title"
                            name="title"
                        >
                    </label>
                    <br>
                    <label>Plain text content:<br>
                        <textarea
                            placeholder="Enter the content in plain text"
                            name="text_content"
                            rows="20"
                            cols="50"
                        ></textarea>
                    </label>
                    <br>
                    <label>HTML content:<br>
                        <textarea
                            placeholder="Enter the content in HTML format"
                            name="html_content"
                            rows="20"
                            cols="50"
                        ></textarea>
                    </label>
                    <br>
                    <input hidden type="text" name="idempotency_key" value="{}">
                    <button type="submit">Publish</button>
                </form>
            </body>
       </html>
            "#,
            html_msg, idempotency_key
        )))
}
