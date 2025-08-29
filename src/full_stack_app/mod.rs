mod backend;
mod database;
mod frontend;
mod reverse_proxy;

use std::ops::Deref;

use backend::Backend;
use database::Database;
use frontend::Frontend;
use reverse_proxy::ReverseProxy;

// TODO: Consider using macros for stack creation to reduce boilerplate
// or use a more structured approach to define resources.
//
// Typically, you would have a more complex structure here
// that includes all the resources and their configurations.
//
// Also you could uses self instead of passing stack around.
pub struct FullStackApp;

impl FullStackApp {
    // TODO: Instead of passing github_access_token we could extend the stack properties with macros
    pub fn new(
        app: &aws_cdk_lib::App,
        github_access_token: String,
        lambda_zip_path: String,
        frontend_github_repository: String,
    ) -> Self {
        let stack =
            aws_cdk_lib::Stack::new(Some(app), Some("MyDataProjectStack".to_string()), None);

        let database = Database::new(&stack);

        let frontend = Frontend::new(&stack, github_access_token, frontend_github_repository);
        let backend = Backend::new(&stack, database.endpoint, lambda_zip_path);

        let _reverse_proxy = ReverseProxy::new(
            &stack,
            backend.lambda_function_url.deref(),
            frontend.amplify_domain,
        );

        Self
    }
}
