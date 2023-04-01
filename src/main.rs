use actix_web::web::Data;
use lambda_web::actix_web::{self, App, HttpServer};
use lambda_web::{is_running_on_lambda, run_actix_on_lambda, LambdaError};
use playground_api::config::api_docs::ApiDoc;
use playground_api::config::telemetry::{get_subscriber, init_subscriber};
use playground_api::controller::account_controller::list_accounts;
use playground_api::repository::ConfigProvider;
use playground_api::service::AccountService;
use tracing_actix_web::TracingLogger;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[actix_web::main]
async fn main() -> Result<(), LambdaError> {
    let subscriber = get_subscriber("playground-api".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let config_provider = ConfigProvider::default().provide().await;
    let factory = move || {
        App::new()
            .wrap(TracingLogger::default())
            .service(list_accounts)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
            .app_data(Data::new(AccountService::new(&config_provider)))
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
