use aws_cdk_lib::aws_lambda::{self, Code, CodeTrait, RuntimeTrait};

pub struct Backend;

impl Backend {
    pub fn new(stack: &aws_cdk_lib::Stack) -> Self {
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

        let _lambda = aws_lambda::Function::new(stack, "MyDataProjectLambda".to_string(), 
        aws_lambda::FunctionProps {
            // TODO: Generate Rust-style statics
            runtime: aws_lambda::Runtime::NODEJS_18_X(),
            handler: "index.handler".to_string(),
            code: inline_code,
            ..Default::default()
        });

        Self
    }
}