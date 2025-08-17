use aws_cdk_lib::aws_elasticbeanstalk::{self, CfnApplicationTrait};
use aws_cdk_lib::aws_lambda;

pub struct Backend;

impl Backend {
    pub fn new(stack: &aws_cdk_lib::Stack) -> Self {
        // TODO: Use lambda
        let lambda = aws_lambda::CfnFunction::new(stack, "MyDataProjectLambda".to_string(), aws_lambda::CfnFunctionProps {
            // runtime: aws_lambda::Runtime::PYTHON_3_8, // TODO: Docker or Rust if exists in 2025
            handler: Some("app.handler".to_string()),
            // code: aws_lambda::Code::from_asset("path/to/lambda/code"), // TODO: Use static method if exists
            ..Default::default()
        });

        // TODO: Use Elastic Beanstalk if needed
        // let elastic_beanstalk = aws_elasticbeanstalk::CfnApplication::new(stack, "MyDataProjectElasticBeanstalk".to_string(), None);

        // let _environment = aws_elasticbeanstalk::CfnEnvironment::new(stack, "MyDataProjectEnvironment".to_string(), aws_elasticbeanstalk::CfnEnvironmentProps {
        //     application_name: elastic_beanstalk.get_application_name().unwrap(),
        //     solution_stack_name: Some("MyDataProjectBackend".to_string()),
        //     ..Default::default()
        // });

        Self
    }
}