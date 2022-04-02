terraform {
  required_version = ">= 1.1"
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = ">= 4.8"
    }
    heroku = {
      source  = "heroku/heroku"
      version = ">= 5.0"
    }
  }
}
