mod helpers;

use aws_cdk_lib::App;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = App::new(None);

    helpers::synth_app(app)?;

    Ok(())
}
