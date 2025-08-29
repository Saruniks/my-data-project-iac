use aws_cdk_lib::{
    aws_iam,
    aws_lambda::{self, CodeTrait, FunctionTrait, FunctionUrlTrait, IFunctionTrait, RuntimeTrait},
};

pub struct Backend {
    pub lambda_function_url: Box<dyn FunctionUrlTrait>,
}

impl Backend {
    pub fn new(
        stack: &aws_cdk_lib::Stack,
        database_endpoint: String,
        lambda_zip_path: String,
    ) -> Self {
        let bundled_code = aws_lambda::Code::from_asset(lambda_zip_path.to_string(), None);

        let lambda = aws_lambda::Function::new(
            stack,
            "MyDataProjectLambda".to_string(),
            aws_lambda::FunctionProps {
                // TODO: Generate and use Rust-style statics
                runtime: aws_lambda::Runtime::NODEJS_18_X(),
                handler: "index.handler".to_string(),
                code: bundled_code,
            },
        );

        // TODO: scope down permissions
        lambda.add_to_role_policy(&aws_iam::PolicyStatement::new(Some(
            aws_iam::PolicyStatementProps {
                effect: Some(aws_iam::Effect::Allow),
                actions: Some(vec!["dsql:*".to_string()]),
                resources: Some(vec!["*".to_string()]),
                ..Default::default()
            },
        )));

        lambda.add_environment("DATABASE_ENDPOINT".to_string(), database_endpoint, None);

        let lambda_function_url = lambda.add_function_url(Some(aws_lambda::FunctionUrlOptions {
            auth_type: Some(aws_lambda::FunctionUrlAuthType::None),
            ..Default::default()
        }));

        Self {
            lambda_function_url,
        }
    }
}
