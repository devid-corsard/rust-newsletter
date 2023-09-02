use actix_web::{http::header::ContentType, HttpResponse};
use actix_web_flash_messages::IncomingFlashMessages;

pub async fn newsletters_form(
    flash_messages: IncomingFlashMessages,
) -> Result<HttpResponse, actix_web::Error> {
    let idempotency_key = uuid::Uuid::new_v4().to_string();
    let mut html_messages = String::new();
    for message in flash_messages.iter() {
        html_messages.push_str(&format!("<p><i>{}</i></p>", message.content()));
    }
    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#" <!doctype html>
<html lang="en">
    <head>
        <title>New newsletter issue</title>
        <meta charset="UTF-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <!-- <link href="css/style.css" rel="stylesheet"> -->
        <style>
            html {{
                color-scheme: dark;
            }}
        </style>
    </head>
    <body>
        <h1>New newsletter</h1>
        {html_messages}
        <form action="/admin/newsletters" method="post">
            <label
                >Title
                <input type="text" placeholder="Enter title" name="title" />
            </label>
            <br />
            <label
                >Text content
                <textarea
                    placeholder="Enter text content"
                    name="text_content"
                    cols="50"
                    rows="20"
                ></textarea>
            </label>
            <br />
            <label
                >HTML content
                <textarea
                    placeholder="Enter html content"
                    name="html_content"
                    cols="50"
                    rows="20"
                ></textarea>
            </label>
            <br />
            <input hidden type="text" name="idempotency_key" value="{idempotency_key}">
            <button type="submit">Publish</button>
        </form>
        <p><a href="/admin/dashboard">&lt;- Back</a></p>
    </body>
</html>
"#
        )))
}
