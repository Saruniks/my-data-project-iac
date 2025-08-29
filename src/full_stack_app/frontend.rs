use aws_cdk_lib::aws_amplify::{self, CfnAppProps, CfnAppTrait, CfnBranchProps};

pub struct Frontend {
    pub amplify_domain: String,
}

impl Frontend {
    pub fn new(stack: &aws_cdk_lib::Stack, github_access_token: String, github_repository: String) -> Self {
        let amplify = aws_amplify::CfnApp::new(stack, "MyDataProjectAmplifyInstance".to_string(), aws_amplify::CfnAppProps {
            name: "MyDataProjectFrontend".to_string(),
            access_token: Some(github_access_token),
            repository: Some(github_repository),
            platform: Some("WEB_COMPUTE".to_string()), // TODO: Use generated constants

            ..CfnAppProps::default()
        });

        let branch_name = "main".to_string();

        let _branch = aws_amplify::CfnBranch::new(stack, "MyDataProjectAmplifyBranch".to_string(), aws_amplify::CfnBranchProps {
            app_id: amplify.get_attr_app_id(),
            branch_name: branch_name.clone(),
            // TODO: fix: Doesn't trigger "run job" on first cdk deployment
            enable_auto_build: Some(serde_json::Value::Bool(true)),  // json value for now because of bool | IResolvable
            ..CfnBranchProps::default()
        });

        let amplify_domain = format!("{}.{}", branch_name, amplify.get_attr_default_domain());

        Self {
            amplify_domain,
        }
    }
}
