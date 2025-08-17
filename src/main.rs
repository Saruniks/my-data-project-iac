use aws_cdk_lib::{aws_amplify, aws_s3, aws_elasticbeanstalk, aws_cloudfront};

mod helpers;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = aws_cdk_lib::App::new(None);

    let stack = aws_cdk_lib::Stack::new(Some(&app), Some("MyDataProjectStack".to_string()), None);

    let _bucket = aws_s3::Bucket::new(&stack, "MyDataProjectBucket".to_string(), None);

    let _cloudfront = aws_cloudfront::CfnDistribution::new(&stack, "MyDataProjectCloudFront".to_string(), aws_cloudfront::CfnDistributionProps {
        ..Default::default()
    });

    let _amplify = aws_amplify::CfnApp::new(&stack, "MyDataProjectAmplifyInstance".to_string(), aws_amplify::CfnAppProps {
        ..Default::default()
    });

    let _elastic_beanstalk = aws_elasticbeanstalk::CfnApplication::new(&stack, "MyDataProjectElasticBeanstalk".to_string(), None);

    helpers::synth_app(app)?;

    Ok(())
}
