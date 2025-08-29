#!/bin/bash
set -e

REGION=${AWS_REGION:-"us-east-1"}
STACK_NAME="MyDataProjectStack"

echo "Deleting CloudFormation stack $STACK_NAME from region: $REGION"

aws cloudformation delete-stack --stack-name $STACK_NAME --region $REGION

echo "Waiting for stack deletion to complete..."
aws cloudformation wait stack-delete-complete --stack-name $STACK_NAME --region $REGION

echo "Stack deletion complete!"
