use actix_web::web;

use crate::email_client::GenericEmailService;

#[tracing::instrument(name = "Sending OTP", skip(email_service), fields(email_service))]
pub async fn send_email_background(
    email_service: web::Data<dyn GenericEmailService>,
    identifier: String,
) {
    // Perform the email sending in the background
    let _response = email_service
        .send_html_email(&identifier, "SANU", "mango".to_owned())
        .await;

    // Optionally handle the response or log any errors
    match _response {
        Ok(_) => tracing::info!("Email sent successfully in the background"),
        Err(err) => tracing::error!("Failed to send email in the background: {:?}", err),
    }
}

#[tracing::instrument(name = "Get Message Template", skip(), fields())]
pub async fn get_message_template() {}
