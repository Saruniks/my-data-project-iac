use aws_cdk_lib::aws_lambda::{self, Code, CodeTrait, IFunctionTrait, RuntimeTrait};
use aws_cdk_lib::aws_lambda::FunctionUrlTrait;
use std::ops::Deref;
use aws_cdk_lib::aws_lambda::FunctionTrait;
pub struct Backend {
    pub lambda_url: String,
}

impl Backend {
    pub fn new(stack: &aws_cdk_lib::Stack, database_endpoint: String) -> Self {
        let lambda_code = r#"
exports.handler = async (event) => {
    console.log('Event:', JSON.stringify(event, null, 2));
    
    return {
        statusCode: 200,
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify({
            message: 'Hello from Lambda!',
            timestamp: new Date().toISOString(),
            event: event
        })
    };
};
"#;

        let inline_code = Code::from_inline(lambda_code.to_string());

        let lambda = aws_lambda::Function::new(stack, "MyDataProjectLambda".to_string(), 
        aws_lambda::FunctionProps {
            // TODO: Generate Rust-style statics
            runtime: aws_lambda::Runtime::NODEJS_18_X(),
            handler: "index.handler".to_string(),
            code: inline_code,
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