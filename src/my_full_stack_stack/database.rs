use aws_cdk_lib::aws_rds;
pub struct Database;

impl Database {
    pub fn new(stack: &aws_cdk_lib::Stack) -> Self {
        let _aurora = aws_rds::CfnCustomDBEngineVersion::new(stack, "MyDataProjectAurora".to_string(), aws_rds::CfnCustomDBEngineVersionProps {
            engine_version: "some-engine-version".to_string(), // TODO: Replace with actual version
            // engine_version: aws_rds::AuroraPostgresEngineVersion::get_VER_16_4().to_string(), // TODO: Make this static-global rust-like
            ..Default::default()
        });

        Self
    }
}