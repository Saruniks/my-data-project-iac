#!/bin/bash
set -e

REGION=${AWS_REGION:-"us-east-1"}
STACK_NAME="MyDataProjectStack"

echo "Publishing CDK assets to region: $REGION"
cdk-assets --path cdk.out publish

echo "Deploying CloudFormation stack $STACK_NAME..."
aws cloudformation deploy \
  --template-file cdk.out/${STACK_NAME}.template.json \
  --stack-name $STACK_NAME \
  --capabilities CAPABILITY_IAM CAPABILITY_NAMED_IAM \
  --region $REGION

echo "Deployment completed successfully!"