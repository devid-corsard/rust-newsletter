use uuid::Uuid;

use crate::{helpers::spawn_app, login::assert_is_redirect_to};

#[tokio::test]
async fn you_must_be_logged_in_to_see_the_change_password_form() {
    let app = spawn_app().await;
    let response = app.get_change_password().await;
    assert_is_redirect_to(&response, "/login");
}

#[tokio::test]
async fn you_must_be_logged_in_to_change_your_password() {
    let app = spawn_app().await;
    let new_password = Uuid::new_v4().to_string();
    let body = serde_json::json!({
        "current_password":Uuid::new_v4().to_string(),
        "new_password":&new_password,
        "new_password_check":new_password,
    });
    let response = app.post_change_password(&body).await;
    assert_is_redirect_to(&response, "/login");
}

#[tokio::test]
async fn new_password_fields_must_match() {
    let app = spawn_app().await;
    let new_password = Uuid::new_v4().to_string();
    let wrong_new_password = Uuid::new_v4().to_string();
    let login_body = serde_json::json!({
        "username": &app.test_user.username,
        "password": &app.test_user.password,
    });
    app.post_login(&login_body).await;
    let change_password_body = serde_json::json!({
        "current_password": &app.test_user.password,
        "new_password": &new_password,
        "new_password_check": &wrong_new_password,
    });
    let response = app.post_change_password(&change_password_body).await;
    assert_is_redirect_to(&response, "/admin/password");
    let html_page = app.get_change_password_html().await;
    assert!(html_page.contains("<p><i>New passwords doesn't match.</i></p>"));
}

#[tokio::test]
async fn current_password_must_be_valid() {
    let app = spawn_app().await;
    let new_password = Uuid::new_v4().to_string();
    let wrong_password = Uuid::new_v4().to_string();
    let login_body = serde_json::json!({
        "username": &app.test_user.username,
        "password": &app.test_user.password,
    });
    app.post_login(&login_body).await;
    let change_password_body = serde_json::json!({
        "current_password": &wrong_password,
        "new_password": &new_password,
        "new_password_check": &new_password,
    });
    let response = app.post_change_password(&change_password_body).await;
    assert_is_redirect_to(&response, "/admin/password");
    let html_page = app.get_change_password_html().await;
    assert!(html_page.contains("<p><i>Current password is invalid.</i></p>"));
}

#[tokio::test]
async fn new_password_is_too_short() {
    let app = spawn_app().await;
    let mut new_password = Uuid::new_v4().to_string();
    new_password.truncate(7);
    let login_body = serde_json::json!({
        "username": &app.test_user.username,
        "password": &app.test_user.password,
    });
    let change_password_body = serde_json::json!({
        "current_password": &app.test_user.password,
        "new_password": &new_password,
        "new_password_check": &new_password,
    });
    app.post_login(&login_body).await;
    let response = app.post_change_password(&change_password_body).await;
    assert_is_redirect_to(&response, "/admin/password");
    let html_page = app.get_change_password_html().await;
    assert!(html_page.contains("<p><i>Password must be at least 8 characters long.</i></p>"));
}

#[tokio::test]
async fn changing_password_works() {
    let app = spawn_app().await;
    let new_password = Uuid::new_v4().to_string();
    let login_body = serde_json::json!({
        "username": &app.test_user.username,
        "password": &app.test_user.password,
    });
    let change_password_body = serde_json::json!({
        "current_password": &app.test_user.password,
        "new_password": &new_password,
        "new_password_check": &new_password,
    });
    // LOGIN
    let response = app.post_login(&login_body).await;
    assert_is_redirect_to(&response, "/admin/dashboard");
    // CHANGE PASSWORD
    let response = app.post_change_password(&change_password_body).await;
    assert_is_redirect_to(&response, "/admin/password");
    // FOLLOW THE REDIRECT
    let html_page = app.get_change_password_html().await;
    assert!(html_page.contains("<p><i>Password successfully changed.</i></p>"));
    // logout
    let response = app.post_logout().await;
    assert_is_redirect_to(&response, "/login");
    // FOLLOW THE REDIRECT
    let html_page = app.get_login_html().await;
    assert!(html_page.contains("<p><i>You have successfully logged out.</i></p>"));
    // LOGIN USENG NEW PASSWORD
    let new_login_body = serde_json::json!({
        "username": &app.test_user.username,
        "password": &new_password,
    });
    let response = app.post_login(&new_login_body).await;
    assert_is_redirect_to(&response, "/admin/dashboard");
}
