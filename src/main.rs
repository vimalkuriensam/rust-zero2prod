use std::io::Result;
use zero2prod::run;

#[actix_web::main]
async fn main() -> Result<()> {
    run().await
}
