use aws_cdk_lib::aws_elasticbeanstalk;

pub struct Backend;

impl Backend {
    pub fn new(stack: &aws_cdk_lib::Stack) -> Self {
        let _elastic_beanstalk = aws_elasticbeanstalk::CfnApplication::new(stack, "MyDataProjectElasticBeanstalk".to_string(), None);

        Self
    }
}