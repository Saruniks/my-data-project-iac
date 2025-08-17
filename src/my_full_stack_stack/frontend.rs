use aws_cdk_lib::aws_amplify::{self, CfnAppProps, CfnAppTrait, CfnBranchProps};
use aws_cdk_lib::aws_iam::{self, IRoleTrait};

pub struct Frontend;

impl Frontend {
    pub fn new(stack: &aws_cdk_lib::Stack, github_access_token: String) -> Self {
        // let assumed_by = aws_iam::ServicePrincipal::new("amplify.amazonaws.com".to_string(), None);
        
        // let managed_policy = aws_iam::ManagedPolicy::new(
        //     stack,
        //     "AmplifyServiceRolePolicy".to_string(),
        //     // TODO: Use managed amplify policy or use from_json static method otherwise
        //     Some(aws_iam::ManagedPolicyProps {
        //         managed_policy_name: Some("AmplifyServiceRolePolicy".to_string()),
        //         statements: Some(vec![
        //             Box::new(aws_iam::PolicyStatement::new(
        //                 Some(aws_iam::PolicyStatementProps {
        //                     actions: vec![
        //                         "amplify:CreateApp".to_string(),
        //                         "amplify:UpdateApp".to_string(),
        //                         "amplify:DeleteApp".to_string(),
        //                         "amplify:GetApp".to_string(),
        //                     ],
        //                     resources: vec!["*".to_string()],
        //                     ..Default::default()
        //                 },
        //             )))]),
        //         ..Default::default()
        //     }),
        // );

        // let amplify_service_role = aws_iam::Role::new(
        //     stack,
        //     "AmplifyServiceRole".to_string(),
        //     aws_iam::RoleProps {
        //         assumed_by: Box::new(assumed_by),
        //         managed_policies: Some(vec![
        //             Box::new(managed_policy),
        //         ]),
        //         ..Default::default()
        //     },
        // );

        let _amplify = aws_amplify::CfnApp::new(stack, "MyDataProjectAmplifyInstance".to_string(), aws_amplify::CfnAppProps {
            name: "MyDataProjectFrontend".to_string(), // TODO: Remove name default
            access_token: Some(github_access_token),
            repository: Some("https://github.com/Saruniks/my-data-project-frontend.git".to_string()), 
            iam_service_role: Some(std::env::var("AMPLIFY_IAM_SERVICE_ROLE_ARN").unwrap()), // TODO: Create role and assign here
            platform: Some("WEB_COMPUTE".to_string()), // TODO: Use generated constant
            ..CfnAppProps::default()
        });

        let _branch = aws_amplify::CfnBranch::new(stack, "MyDataProjectAmplifyBranch".to_string(), aws_amplify::CfnBranchProps {
            app_id: _amplify.get_attr_app_id(),
            branch_name: "main".to_string(),
            ..CfnBranchProps::default()
        });

        Self
    }
}
