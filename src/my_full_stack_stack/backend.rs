use aws_cdk_lib::aws_lambda::{self, Code, CodeTrait, IFunctionTrait, RuntimeTrait};
use aws_cdk_lib::aws_s3::BucketTrait;
use aws_cdk_lib::aws_s3_assets;
use aws_cdk_lib::aws_lambda::FunctionUrlTrait;
use std::ops::Deref;
use aws_cdk_lib::aws_lambda::FunctionTrait;
pub struct Backend {
    pub lambda_url: String,
}

impl Backend {
    pub fn new(stack: &aws_cdk_lib::Stack, database_endpoint: String) -> Self {
        let bucket = aws_cdk_lib::aws_s3::Bucket::from_bucket_name(
            stack,
            "MyDataProjectLambdaBucket".to_string(),
            "my-data-project-lambda".to_string(),
        );

        // Use the bundled zip file that includes dependencies
        let bundled_code = aws_lambda::Code::from_bucket(
            bucket.deref(),
            "index.zip".to_string(),
            None,
        );

        let lambda = aws_lambda::Function::new(stack, "MyDataProjectLambda".to_string(), 
        aws_lambda::FunctionProps {
            // TODO: Generate Rust-style statics
            runtime: aws_lambda::Runtime::NODEJS_18_X(),
            handler: "index.handler".to_string(),
            code: bundled_code,
            ..Default::default()
        });

        lambda.add_environment("DATABASE_ENDPOINT".to_string(), database_endpoint, None);

        let function_url = lambda.add_function_url(
            Some(aws_lambda::FunctionUrlOptions {
                auth_type: Some(aws_lambda::FunctionUrlAuthType::None),
                ..Default::default()
            })
        );

        Self {
            lambda_url: FunctionUrlTrait::get_url(function_url.deref()),
        }
    }
}