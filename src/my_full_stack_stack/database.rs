use aws_cdk_lib::aws_dsql;

pub struct Database;

impl Database {
    pub fn new(stack: &aws_cdk_lib::Stack) -> Self {
        let _resource = aws_dsql::CfnCluster::new(
            stack,
            "MyDataProjectDsqlCluster".to_string(),
            Some(aws_dsql::CfnClusterProps {
                ..Default::default()
            }),
        );

        Self
    }
}