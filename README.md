# AWS CDK Rust Playground

This project serves as a playground for testing the initial version of AWS CDK Rust.

## Project Overview

Infrastructure-as-code (IaC) project for a simple full-stack AWS application using the generated Rust bindings for AWS CDK. The components include:

- **Frontend**: AWS Amplify hosting for a web application from a GitHub repository
- **Backend**: AWS Lambda function
- **Database**: Amazon Aurora DSQL database
- **Networking**: CloudFront distribution serving as a reverse proxy

## Current State

This is an experimental project using pre-release Rust bindings for AWS CDK. As such, it demonstrates both the capabilities and current limitations of these bindings. The code contains workarounds and TODOs that highlight areas for future improvement.

## Environment Variables

The following environment variables are required:

```
GITHUB_ACCESS_TOKEN=your_github_token
FRONTEND_GITHUB_REPOSITORY=your_github_repo_url
LAMBDA_ZIP_PATH=path_to_lambda_code_zip
```

## Usage

1. Set up the required environment variables
2. Run the project to generate CloudFormation templates:
   ```
   cargo run
   ```
3. Deploy the stack:
   ```
   ./deploy.sh
   ```
4. To tear down the infrastructure:
   ```
   ./destroy.sh
   ```

## Known Issues and Limitations

- Amplify job needs to be ran manually after initial deployment
