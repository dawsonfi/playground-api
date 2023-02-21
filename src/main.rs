use actix_web::web::Data;
use env_logger::{init_from_env, Env};
use lambda_web::actix_web::{self, App, HttpServer};
use lambda_web::{is_running_on_lambda, run_actix_on_lambda, LambdaError};
use playground_api::controller::account_controller::list_accounts;
use playground_api::repository::account_repository::AccountRepository;
use playground_api::repository::ConfigProvider;

#[actix_web::main]
async fn main() -> Result<(), LambdaError> {
    configure_log();

    let config_provider = ConfigProvider::default().provide().await;
    let factory = move || {
        App::new()
            .service(list_accounts)
            .app_data(Data::new(AccountRepository::new(&config_provider)))
    };

    if is_running_on_lambda() {
        // Run on AWS Lambda
        run_actix_on_lambda(factory).await?;
    } else {
        // Local server
        HttpServer::new(factory)
            .bind(("127.0.0.1", 8080))?
            .run()
            .await?;
    }
    Ok(())
}

fn configure_log() {
    let env = Env::default().filter_or("MY_LOG_LEVEL", "info");
    init_from_env(env);
}
