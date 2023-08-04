use wiremock::{
    matchers::{any, method, path},
    Mock, ResponseTemplate,
};

use crate::helpers::{spawn_app, ConfirmationLinks, TestApp};

#[tokio::test]
async fn newsletters_are_not_delivered_to_unconfirmed_subscribers() {
    let app = spawn_app().await;
    create_unconfirmed_subscriber(&app).await;

    Mock::given(any())
        .respond_with(ResponseTemplate::new(200))
        .expect(0)
        .mount(&app.email_server)
        .await;

    let newsletter_body = serde_json::json!({
        "title":"Newsletter title",
        "content":{
            "html":"<h1>Newsletter html content</h1>",
            "text":"Nesletter plain text content"
        }
    });

    let response = app.post_newsletters(newsletter_body).await;
    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn newsletters_are_delivered_to_confirmed_subscribers() {
    let app = spawn_app().await;
    create_confirmed_subscriber(&app).await;

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&app.email_server)
        .await;

    let newsletter_body = serde_json::json!({
        "title":"Newsletter title",
        "content":{
            "html":"<h1>Newsletter html content</h1>",
            "text":"Nesletter plain text content"
        }
    });

    let response = app.post_newsletters(newsletter_body).await;
    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn newsletters_returns_400_for_invalid_data() {
    let app = spawn_app().await;
    let test_cases = [
        (
            serde_json::json!({
            "content":{
                "html":"<h1>Newsletter html content</h1>",
                "text":"Nesletter plain text content"
            }
            }),
            "missing title",
        ),
        (
            serde_json::json!({"title":"Newsletter title"}),
            "missing content",
        ),
    ];

    for (invalid_body, error) in test_cases {
        let response = app.post_newsletters(invalid_body).await;
        assert_eq!(
            response.status().as_u16(),
            400,
            "Didn't send 400 when body is invalid becouse of {}",
            error
        );
    }
}

async fn create_unconfirmed_subscriber(app: &TestApp) -> ConfirmationLinks {
    let body = "name=devid%20corsard&email=devid_corsard%40gmail.com";

    let _mock_guard = Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .named("Create unconfirmed subscriber")
        .expect(1)
        .mount_as_scoped(&app.email_server)
        .await;

    app.post_subscriptions(body.into())
        .await
        .error_for_status()
        .unwrap();

    let email_request = &app
        .email_server
        .received_requests()
        .await
        .unwrap()
        .pop()
        .unwrap();

    app.get_confirmation_links(&email_request)
}

async fn create_confirmed_subscriber(app: &TestApp) {
    let confirm_link = create_unconfirmed_subscriber(app).await;

    reqwest::get(confirm_link.html)
        .await
        .unwrap()
        .error_for_status()
        .unwrap();
}
