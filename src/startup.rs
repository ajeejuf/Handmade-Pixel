
use crate::routes::{ health_check, home, home1, lessons, login_signup_form, login_signup, confirm, learn_more};
use actix_files::Files;
use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use actix_web::dev::Server;
use std::net::TcpListener;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use tracing_actix_web::TracingLogger;
use crate::configuration::{Settings, DatabaseSettings};
use crate::email_client::EmailClient;

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async  fn build(configuration: Settings) -> Result<Self, std::io::Error> {
        let connection_pool = get_connection_pool(&configuration.database);

        let sender_email = configuration.email_client.sender()
            .expect("Invalid sender email address.");
        let timeout = configuration.email_client.timeout();
        let email_client = EmailClient::new(
            configuration.email_client.base_url,
            sender_email,
            configuration.email_client.authorization_token,
            timeout
        );
        
        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr().unwrap().port();
        let server = run(
            listener,
            connection_pool,
            email_client,
            configuration.application.base_url,
        )?;

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

pub fn get_connection_pool(
    configuration: &DatabaseSettings
) -> PgPool {
    PgPoolOptions::new().connect_lazy_with(configuration.with_db())
}

pub struct ApplicationBaseUrl(pub String);

fn run(
    listener: TcpListener,
    db_pool: PgPool,
    email_client: EmailClient,
    base_url: String,
) -> Result<Server, std::io::Error> {
    let db_pool = Data::new(db_pool);
    let email_client = Data::new(email_client);
    let base_url = Data::new(ApplicationBaseUrl(base_url));
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .service(
                Files::new("/static", "./static")
                    .show_files_listing()
                    .use_last_modified(true)
            )
            .route("/health_check", web::get().to(health_check))
            .route("/", web::get().to(home))
            .route("/1", web::get().to(home1))
            .route("/design", web::get().to(lessons))
            .route("/login-signup", web::get().to(login_signup_form))
            .route("/login-signup", web::post().to(login_signup))
            .route("/login-signup/confirm", web::get().to(confirm))
            .route("/learn-more", web::get().to(learn_more))
            .app_data(db_pool.clone())
            .app_data(email_client.clone())
            .app_data(base_url.clone())
    })
        .listen(listener)?
        .run();

    Ok(server)
}
