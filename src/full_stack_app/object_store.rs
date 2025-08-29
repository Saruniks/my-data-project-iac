use aws_cdk_lib::aws_s3;

pub struct ObjectStore;

impl ObjectStore {
    pub fn new(stack: &aws_cdk_lib::Stack) -> Self {
        let _bucket = aws_s3::Bucket::new(stack, "MyDataProjectBucket".to_string(), None);
        Self
    }
}