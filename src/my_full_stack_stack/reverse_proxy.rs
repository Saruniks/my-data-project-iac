use aws_cdk_lib::aws_cloudfront;
pub struct ReverseProxy;

impl ReverseProxy {
    pub fn new(stack: &aws_cdk_lib::Stack) -> Self {
        let _cloudfront = aws_cloudfront::CfnDistribution::new(stack, "MyDataProjectCloudFront".to_string(), aws_cloudfront::CfnDistributionProps {
            ..Default::default()
        });

        Self
    }
}