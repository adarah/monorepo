terraform {
  backend "s3" {
    bucket         = "tf-state20220402145646679100000001"
    key            = "main.tfstate"
    dynamodb_table = "TerraformStateLock"
    region         = "us-east-1"
  }
}
