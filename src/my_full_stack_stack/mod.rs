mod object_store;
mod reverse_proxy;
mod frontend;
mod backend;
mod database;

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
    pub fn new(app: &aws_cdk_lib::App) -> Self {
        let stack = aws_cdk_lib::Stack::new(Some(app), Some("MyDataProjectStack".to_string()), None);

        let _object_store = ObjectStore::new(&stack);

        let _elastic_beanstalk = Backend::new(&stack);
        let _amplify = Frontend::new(&stack);
        let _reverse_proxy = ReverseProxy::new(&stack);

        let _database = Database::new(&stack);

        Self
    }
}