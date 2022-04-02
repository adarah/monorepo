# Hi there
If you're reading this from my profile you might be confused, but this README is intended for [my main monorepo](https://github.com/Adarah/adarah) which contains the recent projects I've been working on.

## Project overview
Below is a short description of each project. You can get more details in each project's README.
```bash
.
├── infrastructure  # Terraform IaC project to deploy everything else
├── raytracer  # A fun Rust project to learn computer graphics
└── satire-bot # A Go discord bot for my Lost Ark guild
```

## Build
To build and test this whole repository, you will only need bazel installed. I recommend getting it via bazelisk as detailed [here](https://github.com/bazelbuild/bazelisk): 

To build all projects, run:
```bash
bazel build //...
```
This will download all dependencies for all languages and compile them from scratch with bazel. It might take a while to complete for the first time, especially for the rust dependencies, but bazel will cache everything in consecutive runs.

## Test
To test all projects, run:
```bash
bazel build //...
```

## Deploy
<details>
<summary>First time setup </summary>

### Prepare the environment
To deploy this project on your own infrastructure, you will need:
- Terraform 1.1 or later (installation steps [here](https://learn.hashicorp.com/tutorials/terraform/install-cli))
- An AWS account
- A Heroku account

And the following environment variables set:
```bash
export AWS_ACCESS_KEY_ID=
export AWS_SECRET_ACCESS_KEY=
export HEROKU_API_KEY=
export HEROKU_EMAIL=
```

### Bootstrap Terraform
After that, you will need to boostrap the `terraform` S3 backend. You can do that with:
```bash
cd infrastructure/bootstrap && terraform apply
```
This will create an s3 bucket and a dynamodb table which will store and persist the terraform state and the state lock.
</details>

### Deploying everything
Once the initial setup is done, you can deploy all projects to your infrastructure with:
```bash
cd infrastructure && terraform apply
```
