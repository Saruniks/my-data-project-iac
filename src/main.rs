use aws_cdk_lib::{aws_amplify, aws_s3, aws_elasticbeanstalk, aws_cloudfront, aws_rds};

mod helpers;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = aws_cdk_lib::App::new(None);

    let stack = aws_cdk_lib::Stack::new(Some(&app), Some("MyDataProjectStack".to_string()), None);

    let _s3_bucket = aws_s3::Bucket::new(&stack, "MyDataProjectBucket".to_string(), None);

    let _cloudfront = aws_cloudfront::CfnDistribution::new(&stack, "MyDataProjectCloudFront".to_string(), aws_cloudfront::CfnDistributionProps {
        ..Default::default()
    });

    let _amplify = aws_amplify::CfnApp::new(&stack, "MyDataProjectAmplifyInstance".to_string(), aws_amplify::CfnAppProps {
        ..Default::default()
    });

    let _aurora = aws_rds::CfnCustomDBEngineVersion::new(&stack, "MyDataProjectAurora".to_string(), aws_rds::CfnCustomDBEngineVersionProps {
        engine_version: "some-engine-version".to_string(), // TODO: Replace with actual version
        // engine_version: aws_rds::AuroraPostgresEngineVersion::get_VER_16_4().to_string(), // TODO: Make this static-global rust-like
        ..Default::default()
    });

    let _elastic_beanstalk = aws_elasticbeanstalk::CfnApplication::new(&stack, "MyDataProjectElasticBeanstalk".to_string(), None);

    helpers::synth_app(app)?;

    Ok(())
}
