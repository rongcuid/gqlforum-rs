use gqlforum_backend::startup::run;


#[perseus::engine_main]
async fn main() {
    let exit_code = run().await;
    std::process::exit(exit_code);
}
