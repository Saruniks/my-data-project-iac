use aws_cdk_lib::{aws_cloudfront, aws_cloudfront_origins};
pub struct ReverseProxy;

impl ReverseProxy {
    pub fn new(stack: &aws_cdk_lib::Stack, lambda_url: String) -> Self {
        let amplify_origin = aws_cloudfront_origins::HttpOrigin::new(
            // TODO: Replace with your actual Amplify app domain
            "main.d1qn8oe4agx36g.amplifyapp.com".to_string(),
            Some(aws_cloudfront_origins::HttpOriginProps {
                ..Default::default()
            }),
        );

        // let parsed_lambda_url = lambda_url.replace("https://", "");

        let lambda_origin = aws_cloudfront_origins::HttpOrigin::new(
            // parsed_lambda_url,
            "sma22i3njkknnipodmfwkvxpri0gcvgb.lambda-url.us-east-1.on.aws".to_string(),
            Some(aws_cloudfront_origins::HttpOriginProps {
                ..Default::default()
            }),
        );

        let lambda_behavior = aws_cloudfront::BehaviorOptions {
            origin: Box::new(lambda_origin),
            // allowed_methods: aws_cloudfront::AllowedMethods::ALL,
            // cached_methods: aws_cloudfront::CachedMethods::ALL,
            // viewer_protocol_policy: aws_cloudfront::ViewerProtocolPolicy::REDIRECT_TO_HTTPS,
            ..Default::default()
        };

        let _cloudfront = aws_cloudfront::Distribution::new(stack, "MyDataProjectCloudFront".to_string(), aws_cloudfront::DistributionProps {
            default_behavior: aws_cloudfront::BehaviorOptions {
                origin: Box::new(amplify_origin),
                ..Default::default()
            },
            additional_behaviors: Some(std::collections::HashMap::from([
                ("/api/*".to_string(), lambda_behavior),
            ])),
            ..Default::default()
        });

        Self
    }
}