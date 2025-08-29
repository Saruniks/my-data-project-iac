# AWS CDK Rust Playground

This project serves as a **playground** for the initial version of AWS CDK Rust.

## Project Overview

**Infrastructure-as-code (IaC)** project for a simple full-stack AWS application using **AWS CDK Rust**.  

The components include:
- **Frontend**: AWS Amplify hosting for a web app deployed from a GitHub repository  
- **Backend**: AWS Lambda function  
- **Database**: Amazon Aurora DSQL database  
- **Networking**: CloudFront distribution serving as a reverse proxy  

## Environment Variables

The following environment variables are required:

| Variable                     | Description                              |
|------------------------------|------------------------------------------|
| `GITHUB_ACCESS_TOKEN`        | GitHub token for accessing your repo     |
| `FRONTEND_GITHUB_REPOSITORY` | URL of the frontend GitHub repository    |
| `LAMBDA_ZIP_PATH`            | Path to the packaged Lambda code `.zip` |

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

- ⚠️ **Amplify job needs to be run manually after initial deployment**
  - After stack deployment, navigate to AWS Amplify Console
  - Select your application and branch
  - Click "Run build" to start the initial build process
  - Subsequent code pushes will trigger builds automatically
