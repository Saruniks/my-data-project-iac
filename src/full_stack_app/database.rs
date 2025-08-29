use aws_cdk_lib::aws_dsql::{self, CfnClusterTrait};

pub struct Database {
    pub endpoint: String,
}

impl Database {
    pub fn new(stack: &aws_cdk_lib::Stack) -> Self {
        let resource: aws_dsql::CfnCluster = aws_dsql::CfnCluster::new(
            stack,
            "MyDataProjectDsqlCluster".to_string(),
            Some(aws_dsql::CfnClusterProps {
                ..Default::default()
            }),
        );

        // Get the cluster identifier and build the endpoint URL
        // TODO: Check if there's a more idiomatic way to get the endpoint
        let cluster_id = resource.get_attr_identifier();
        let endpoint = format!("{}.dsql.us-east-1.on.aws", cluster_id);
        
        Self { endpoint }
    }
}