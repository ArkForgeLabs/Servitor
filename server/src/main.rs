use actix_jwt_auth_middleware::use_jwt::UseJWTOnApp;
use actix_web::{middleware::Logger, web, App, HttpServer};
use std::thread;

// components
mod database;
mod routesv1;
mod services;
mod utils;

#[derive(Debug)]
pub struct AppState {
    pub services: std::sync::Mutex<services::Services>,
    pub database: database::Database,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv_rs::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let website_path = std::env::var("WEB_FOLDER").unwrap_or("../editor/build".to_string());
    // generate an ed25519 keypair from seed
    let jwt_keypair = ed25519_compact::KeyPair::from_seed(
        ed25519_compact::Seed::from_slice(
            // using the seed from bytes of
            format!(
                "{:x}",
                // md5 hashed
                md5::compute(
                    // JWT secret salt / phrase in the .env
                    std::env::var("JWT_SECRET")
                        .unwrap_or("<EXAMPLE PASSWORD>".to_string())
                        .as_bytes()
                )
            )
            .as_bytes(),
        )
        .expect("Couldn't generate seed for JWT"),
    );

    // access logs are printed with the INFO level so ensure it is enabled by default
    let database = database::Database::new()
        .await
        .expect("Failed to initialize database");

    let app_data = web::Data::new(AppState {
        services: std::sync::Mutex::new(services::Services::new().await),
        database,
    });

    let heartbeat_clone = app_data.clone();
    thread::spawn(move || loop {
        std::thread::sleep(std::time::Duration::from_secs(10));
        let mut services = heartbeat_clone.services.lock().unwrap();
        services.heartbeat();
    });

    let server = HttpServer::new(move || {
        let authority = actix_jwt_auth_middleware::Authority::<
            routesv1::account::VerifiedAccount,
            jwt_compact::alg::Ed25519,
            _,
            _,
        >::new()
        .refresh_authorizer(|| async move { Ok(()) })
        .token_signer(Some(
            actix_jwt_auth_middleware::TokenSigner::new()
                .signing_key(jwt_keypair.sk.clone())
                .algorithm(jwt_compact::alg::Ed25519)
                .build()
                .expect("Couldn't initialize token signer for JWT"),
        ))
        .verifying_key(jwt_keypair.pk.clone())
        .build()
        .expect("Couldn't install JWT Auth middleware");

        App::new()
            .service(
                web::scope("/apiv1_auth")
                    .service(routesv1::account::login)
                    .service(routesv1::get_all)
                    .service(routesv1::account::create_account),
            )
            .use_jwt(
                authority,
                web::scope("/apiv1")
                    .service(routesv1::get_service)
                    .service(routesv1::create_service)
                    .service(routesv1::create), // account
            )
            .service(actix_files::Files::new("/", website_path.as_str()).index_file("index.html"))
            .wrap(Logger::default())
            .app_data(app_data.clone())
    });
    let server = server.bind(("0.0.0.0", 8080))?;

    println!("started server at: http://0.0.0.0:{}", 8080);

    server.run().await
}
