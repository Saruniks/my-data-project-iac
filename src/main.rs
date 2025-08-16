mod helpers;

use aws_cdk_lib::{App, Stack};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = App::new(None);

    let _stack = Stack::new(Some(&app), Some("MyDataProjectStack".to_string()), None);

    helpers::synth_app(app)?;

    Ok(())
}
