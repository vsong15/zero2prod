pub async fn publish_newsletter_form(/* */) -> Result<HttpResponse, actix_web::Error> {
    // [...]
    let idempotency_key = uuid::Uuid::new_v4();
    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#"<!-- ... -->
            <form action="/admin/newsletters" method="post">
            <!-- ... -->
            <input hidden type="text" name="idempotency_key" value="{idempotency_key}">
            <button type="submit">Publish</button>
            </form>
            <!-- ... -->"#,
        )
    ))
}