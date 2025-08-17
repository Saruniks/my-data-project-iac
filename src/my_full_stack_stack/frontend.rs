use aws_cdk_lib::aws_amplify;

pub struct Frontend;

impl Frontend {
    pub fn new(stack: &aws_cdk_lib::Stack) -> Self {
        let _amplify = aws_amplify::CfnApp::new(stack, "MyDataProjectAmplifyInstance".to_string(), aws_amplify::CfnAppProps {
            ..Default::default()
        });

        Self
    }
}
