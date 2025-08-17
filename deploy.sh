set -e

aws cloudformation deploy --template-file cdk.out/MyDataProjectStack.json --stack-name MyDataProjectStack --capabilities CAPABILITY_IAM --region us-east-1
