use aws_cdk_lib::aws_amplify::{self, CfnAppTrait};

pub struct Frontend;

impl Frontend {
    pub fn new(stack: &aws_cdk_lib::Stack, github_access_token: String) -> Self {
        let _amplify = aws_amplify::CfnApp::new(stack, "MyDataProjectAmplifyInstance".to_string(), aws_amplify::CfnAppProps {
            access_token: Some(github_access_token),
            repository: Some("https://github.com/Saruniks/my-data-project-frontend.git".to_string()),
            ..Default::default()
        });

        let _branch = aws_amplify::CfnBranch::new(stack, "MyDataProjectAmplifyBranch".to_string(), aws_amplify::CfnBranchProps {
            app_id: _amplify.get_attr_app_id(),
            branch_name: "main".to_string(),
            ..Default::default()
        });

        Self
    }
}
