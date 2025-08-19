use aws_cdk_lib::{aws_cloudfront, aws_cloudfront_origins};
pub struct ReverseProxy;

impl ReverseProxy {
    pub fn new(stack: &aws_cdk_lib::Stack) -> Self {
        let amplify_origin = aws_cloudfront_origins::HttpOrigin::new(
            // TODO: Replace with your actual Amplify app domain
            "main.d1qn8oe4agx36g.amplifyapp.com".to_string(),
            Some(aws_cloudfront_origins::HttpOriginProps {
                ..Default::default()
            }),
        );

        let _cloudfront = aws_cloudfront::Distribution::new(stack, "MyDataProjectCloudFront".to_string(), aws_cloudfront::DistributionProps {
            default_behavior: aws_cloudfront::BehaviorOptions {
                origin: Box::new(amplify_origin),
                ..Default::default()
            },
            ..Default::default()
        });

        Self
    }
}