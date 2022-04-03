terraform {
  backend "s3" {
    bucket         = "tf-state20220403033934932000000001"
    key            = "main.tfstate"
    dynamodb_table = "TerraformStateLock"
    region         = "us-east-1"
  }
}
