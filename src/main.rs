mod helpers;
mod my_full_stack_stack;

use my_full_stack_stack::MyFullStackStack;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let github_access_token = std::env::var("GITHUB_ACCESS_TOKEN")
        .expect("GITHUB_ACCESS_TOKEN environment variable is not set");

    let app = aws_cdk_lib::App::new(None);

    let _my_stack = MyFullStackStack::new(&app, github_access_token);

    helpers::synth_app(app)?;

    Ok(())
}
