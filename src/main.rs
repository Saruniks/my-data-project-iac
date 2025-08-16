use aws_cdk_lib::{aws_amplify::{self, CfnAppProps}, aws_s3};

mod helpers;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = aws_cdk_lib::App::new(None);

    let stack = aws_cdk_lib::Stack::new(Some(&app), Some("MyDataProjectStack".to_string()), None);

    let _bucket = aws_s3::Bucket::new(&stack, "MyDataProjectBucket".to_string(), None);

    let _amplify = aws_amplify::CfnApp::new(&stack, "MyDataProjectAmplifyInstance".to_string(), CfnAppProps {
        ..Default::default()
    });

    helpers::synth_app(app)?;

    Ok(())
}
