mod object_store;
mod reverse_proxy;
mod frontend;
mod backend;
mod database;

use std::ops::Deref;

use object_store::ObjectStore;
use reverse_proxy::ReverseProxy;
use frontend::Frontend;
use backend::Backend;
use database::Database;

// TODO: Consider using macros for stack creation to reduce boilerplate
// or use a more structured approach to define resources.
//
// Typically, you would have a more complex structure here
// that includes all the resources and their configurations.
//
// Also you could use self instead of passing stack around.
pub struct MyFullStackStack;

impl MyFullStackStack {
    // TODO: Instead of passing github_access_token we could extend the stack properties with macros
    pub fn new(app: &aws_cdk_lib::App, github_access_token: String, lambda_zip_path: String, frontend_github_repository: String) -> Self {
        let stack = aws_cdk_lib::Stack::new(Some(app), Some("MyDataProjectStack".to_string()), None);

        let _object_store = ObjectStore::new(&stack);

        let database = Database::new(&stack);

        // let _elastic_beanstalk = Backend::new(&stack);
        let frontend = Frontend::new(&stack, github_access_token, frontend_github_repository);
        let backend = Backend::new(&stack, database.endpoint, lambda_zip_path);

        let _reverse_proxy = ReverseProxy::new(&stack, backend.lambda_function_url.deref(), frontend.amplify_domain);

        Self
    }
}