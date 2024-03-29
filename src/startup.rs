use crate::configuration::{DatabaseSettings, RedisSettings, SecretSetting, UserSettings};
use crate::email_client::{DummyEmailClient, GenericEmailService, SmtpEmailClient};
use crate::middleware::SaveRequestResponse;
use crate::redis::RedisClient;
// use crate::middleware::tracing_middleware;

use crate::routes::main_route;
// use actix_session::storage::RedisSessionStore;
// use actix_session::SessionMiddleware;
// use actix_web::cookie::Key;
use actix_web::dev::Server;
// use actix_web::middleware::Logger;
use crate::configuration::Settings;

use actix_web::{web, App, HttpServer};
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::net::TcpListener;
use std::sync::Arc;
use tracing_actix_web::TracingLogger;
pub struct Application {
    port: u16,
    server: Server,
}
impl Application {
    // We have converted the `build` function into a constructor for
    // `Application`.
    pub async fn build(configuration: Settings) -> Result<Self, anyhow::Error> {
        let connection_pool = get_connection_pool(&configuration.database);
        // UNCOMMENT BELOW CODE TO ENABLE EMAIL SERVICE
        // let email_pool = Arc::new(
        //     SmtpEmailClient::new(configuration.email_client)
        //         .expect("Failed to create SmtpEmailClient"),
        // );
        let email_pool =
            Arc::new(DummyEmailClient::new().expect("Failed to create SmtpEmailClient"));
        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
        let redis_obj = RedisClient::new(configuration.redis).await?;
        println!("Listening {}", address);
        let listener = TcpListener::bind(&address)?;
        let port = listener.local_addr().unwrap().port();
        let server = run(
            listener,
            connection_pool,
            email_pool,
            configuration.application.workers,
            configuration.secret,
            redis_obj,
            configuration.user,
        )
        .await?;
        // We "save" the bound port in one of `Application`'s fields
        Ok(Self { port, server })
    }
    pub fn port(&self) -> u16 {
        self.port
    }
    // A more expressive name that makes it clear that
    // this function only returns when the application is stopped.
    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

pub fn get_connection_pool(configuration: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.with_db())
}

async fn run(
    listener: TcpListener,
    db_pool: PgPool,
    email_obj: Arc<dyn GenericEmailService>,
    workers: usize,
    secret: SecretSetting,
    redis_client: RedisClient,
    user_setting: UserSettings,
) -> Result<Server, anyhow::Error> {
    let db_pool = web::Data::new(db_pool);
    let email_client: web::Data<dyn GenericEmailService> = web::Data::from(email_obj);
    let secret_obj = web::Data::new(secret);
    let user_setting_obj = web::Data::new(user_setting);
    // let _secret_key = Key::from(hmac_secret.expose_secret().as_bytes());
    let redis_app = web::Data::new(redis_client);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .wrap(SaveRequestResponse)
            // .wrap(ErrorHandlers::new().handler(StatusCode::BAD_REQUEST, add_error_header))
            // .wrap(Logger::default())  // for minimal logs
            // Register the connection as part of the application state
            // .wrap_fn(tracing_middleware)
            .app_data(db_pool.clone())
            .app_data(email_client.clone())
            .app_data(secret_obj.clone())
            .app_data(user_setting_obj.clone())
            .app_data(redis_app.clone())
            .configure(main_route)
    })
    .workers(workers)
    .listen(listener)?
    .run();

    Ok(server)
}
