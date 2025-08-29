mod full_stack_app;
mod helpers;

use full_stack_app::FullStackApp;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let github_access_token = std::env::var("GITHUB_ACCESS_TOKEN")
        .expect("GITHUB_ACCESS_TOKEN environment variable is not set");

    let lambda_zip_path =
        std::env::var("LAMBDA_ZIP_PATH").expect("LAMBDA_ZIP_PATH environment variable is not set");

    let frontend_github_repository = std::env::var("FRONTEND_GITHUB_REPOSITORY")
        .expect("FRONTEND_GITHUB_REPOSITORY environment variable is not set");

    let app = aws_cdk_lib::App::new(None);

    let _my_stack = FullStackApp::new(
        &app,
        github_access_token,
        lambda_zip_path,
        frontend_github_repository,
    );

    helpers::synth_app(app)?;

    Ok(())
}
