terraform {
  required_version = "~>1.1"

  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 4.8"
    }
    google = {
      source  = "hashicorp/google"
      version = "~> 4.15"
    }
  }
}
