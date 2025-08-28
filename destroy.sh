set -e

aws cloudformation delete-stack --stack-name MyDataProjectStack --region us-east-1

echo "Waiting for stack deletion to complete..."
aws cloudformation wait stack-delete-complete --stack-name MyDataProjectStack --region us-east-1

echo "Stack deletion complete!"
