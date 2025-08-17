use aws_cdk_lib::aws_s3;

pub struct ObjectStore {
    bucket: aws_s3::Bucket,
}

impl ObjectStore {
    pub fn new(stack: &aws_cdk_lib::Stack) -> Self {
        let bucket = aws_s3::Bucket::new(stack, "MyDataProjectBucket".to_string(), None);
        Self { bucket }
    }
}