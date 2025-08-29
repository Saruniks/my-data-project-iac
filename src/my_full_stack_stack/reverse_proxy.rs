use aws_cdk_lib::{aws_cloudfront::{self, CachePolicyTrait, DistributionTrait, OriginRequestPolicyTrait}, aws_cloudfront_origins, aws_lambda::{FunctionTrait, FunctionUrlTrait}};

pub struct ReverseProxy;

impl ReverseProxy {
    pub fn new(stack: &aws_cdk_lib::Stack, lambda_function_url: &dyn FunctionUrlTrait, amplify_url: String) -> Self {
        let amplify_origin = aws_cloudfront_origins::HttpOrigin::new(
            // TODO: Replace with your actual Amplify app domain
            amplify_url.clone(),
            Some(aws_cloudfront_origins::HttpOriginProps {
                ..Default::default()
            }),
        );

        let lambda_origin = aws_cloudfront_origins::FunctionUrlOrigin::new(lambda_function_url, None);

        let cloudfront = aws_cloudfront::Distribution::new(stack, "MyDataProjectCloudFront".to_string(), aws_cloudfront::DistributionProps {
            default_behavior: aws_cloudfront::BehaviorOptions {
                origin: Box::new(amplify_origin),
            },
            ..Default::default()
        });

        cloudfront.add_behavior("/api/*".to_string(), &lambda_origin, Some(aws_cloudfront::AddBehaviorOptions {
            cache_policy: Some(aws_cloudfront::CachePolicy::CACHING_DISABLED()),
            origin_request_policy: Some(aws_cloudfront::OriginRequestPolicy::ALL_VIEWER_EXCEPT_HOST_HEADER()),
            ..Default::default()
            
        }));


        Self
    }
}