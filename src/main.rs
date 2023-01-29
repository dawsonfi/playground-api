mod controller;

use lambda_web::actix_web::{self, App, HttpServer};
use lambda_web::{is_running_on_lambda, run_actix_on_lambda, LambdaError};
use controller::account_controller::list_accounts;

#[actix_web::main]
async fn main() -> Result<(), LambdaError> {
    let factory = move || App::new().service(list_accounts);

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
