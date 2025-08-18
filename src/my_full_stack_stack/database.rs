use aws_cdk_lib::aws_rds::{self, AuroraPostgresEngineVersionTrait, DatabaseClusterEngineTrait};
pub struct Database;

impl Database {
    pub fn new(stack: &aws_cdk_lib::Stack) -> Self {
        let aurora_version = aws_rds::AuroraPostgresEngineVersion::VER_17_4();

        // Create VPC with 3 subnets (Public, Private, Isolated)
        // const vpc = new Vpc(this, `${prefix}-vpc`, {
        //     maxAzs: 2,
        //     natGateways: 1,
        //     subnetConfiguration: [
        //         {
        //         cidrMask: 24,
        //         name: 'isolated',
        //         subnetType: SubnetType.PRIVATE_ISOLATED,
        //         },
        //         {
        //         cidrMask: 24,
        //         name: 'private',
        //         subnetType: SubnetType.PRIVATE_WITH_EGRESS,
        //         },
        //         {
        //         cidrMask: 24,
        //         name: 'Public',
        //         subnetType: SubnetType.PUBLIC,
        //         mapPublicIpOnLaunch: false,
        //         },
        //     ],
        // });

        let vpc = aws_cdk_lib::aws_ec2::Vpc::new(stack, "MyDataProjectVPC".to_string(), None
        // aws_cdk_lib::aws_ec2::VpcProps {
        //     max_azs: 2,
        //     nat_gateways: 1,
        //     subnet_configuration: vec![
        //         aws_cdk_lib::aws_ec2::SubnetConfiguration {
        //             cidr_mask: 24,
        //             name: "isolated".to_string(),
        //             subnet_type: aws_cdk_lib::aws_ec2::SubnetType::PrivateIsolated,
        //         },
        //         aws_cdk_lib::aws_ec2::SubnetConfiguration {
        //             cidr_mask: 24,
        //             name: "private".to_string(),
        //             subnet_type: aws_cdk_lib::aws_ec2::SubnetType::PrivateWithEgress,
        //         },
        //         aws_cdk_lib::aws_ec2::SubnetConfiguration {
        //             cidr_mask: 24,
        //             name: "Public".to_string(),
        //             subnet_type: aws_cdk_lib::aws_ec2::SubnetType::Public,
        //             map_public_ip_on_launch: false,
        //         },
        //     ],
        // }
        );

        // TODO: What are the ways to create a writer instance?
        let writer = aws_rds::IClusterInstance {
            jsii_object_ref: "aws-cdk-lib.aws_rds.IClusterInstance@10011".to_string(), // Placeholder until we have a proper way to create instances
        };

        let _aurora_cluster = aws_rds::DatabaseCluster::new(stack, "MyDataProjectAuroraCluster".to_string(), aws_rds::DatabaseClusterProps {
            engine: aws_rds::DatabaseClusterEngine::aurora_postgres(
                aws_rds::AuroraPostgresClusterEngineProps { version: aurora_version },
            ),
            vpc: Some(Box::new(vpc)),
            writer: Some(Box::new(writer)),
            ..Default::default()
        });

        // let _aurora = aws_rds::CfnCustomDBEngineVersion::new(stack, "MyDataProjectAurora".to_string(), aws_rds::CfnCustomDBEngineVersionProps {
        //     // TODO: Consider having Rust-like static global for this
        //     engine_version: aurora_version.to_string(),
        //     // engine_version: aws_rds::AuroraPostgresEngineVersion::get_VER_16_4().to_string(), // TODO: Make this static-global rust-like
        //     ..Default::default()
        // });

        Self
    }
}