#[actix_web::test]
async fn health_check_works() {
    // Arrange
    spawn_app();

    // Act
    let res = reqwest::get("http://localhost:8081/health_check")
        .await
        .unwrap();

    // Assert
    assert_eq!(res.status(), 200);
    assert_eq!(res.text().await.unwrap(), "");
}

fn spawn_app() {
    let server = zero2prod::run().expect("Failed to bind address!");

    let _ = tokio::spawn(server);
}
