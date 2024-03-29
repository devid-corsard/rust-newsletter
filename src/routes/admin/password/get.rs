use actix_web::{http::header::ContentType, HttpResponse};
use actix_web_flash_messages::IncomingFlashMessages;

pub async fn change_password_form(
    flash_messages: IncomingFlashMessages,
) -> Result<HttpResponse, actix_web::Error> {
    let mut html_messages = String::new();
    for message in flash_messages.iter() {
        html_messages.push_str(&format!("<p><i>{}</i></p>", message.content()));
    }
    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#"<!doctype html>
<html lang="en">
    <head>
        <title>Change Password</title>
        <meta charset="UTF-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <!-- <link href="css/style.css" rel="stylesheet"> -->
    </head>
    <body>
        {html_messages}
        <form action="/admin/password" method="post">
            <label
                >Current password
                <input
                    type="password"
                    placeholder="Enter current password"
                    name="current_password"
                />
            </label>
            <br />
            <label
                >New password
                <input
                    type="password"
                    placeholder="Enter new password"
                    name="new_password"
                />
            </label>
            <br />
            <label
                >Confirm new password
                <input
                    type="password"
                    placeholder="Type the new password again"
                    name="new_password_check"
                />
            </label>
            <br />
            <button type="submit">Change password</button>
        </form>
        <p><a href="/admin/dashboard">&lt;- Back</a></p>
    </body>
</html>"#
        )))
}
