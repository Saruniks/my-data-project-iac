mod helpers;

use aws_cdk_lib::{aws_s3::Bucket, App, Stack};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = App::new(None);

    let stack = Stack::new(Some(&app), Some("MyDataProjectStack".to_string()), None);

    let _bucket = Bucket::new(&stack, "MyDataBucket".to_string(), None);

    helpers::synth_app(app)?;

    Ok(())
}
