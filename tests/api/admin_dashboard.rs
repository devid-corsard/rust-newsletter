use crate::{helpers::spawn_app, login::assert_is_redirect_to};

#[tokio::test]
async fn you_must_be_logged_in_to_access_the_admin_dashboard() {
    let app = spawn_app().await;
    let response = app.get_admin_dashboard().await;
    assert_is_redirect_to(&response, "/login");
}

#[tokio::test]
async fn logout_clears_session_state() {
    let app = spawn_app().await;
    let login_body = serde_json::json!({
        "username": &app.test_user.username,
        "password": &app.test_user.password,
    });
    // LOGIN
    let response = app.post_login(&login_body).await;
    assert_is_redirect_to(&response, "/admin/dashboard");
    // FOLLOW THE REDIRECT
    let html_page = app.get_admin_dashboard_html().await;
    assert!(html_page.contains(&format!("Welcome {}", app.test_user.username)));
    // LOGOUT
    let response = app.post_logout().await;
    assert_is_redirect_to(&response, "/login");
    // FOLLOW THE REDIRECT
    let html_page = app.get_login_html().await;
    assert!(html_page.contains("<p><i>You have successfully logged out.</i></p>"));
    // ATTEMPT TO LOAD ADMIN PANEL
    let response = app.get_admin_dashboard().await;
    assert_is_redirect_to(&response, "/login");
}
