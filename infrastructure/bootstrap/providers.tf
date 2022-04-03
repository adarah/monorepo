provider "aws" {
  region = "us-east-1"
  default_tags {
    tags = {
      app = "terraform-bootstrap"
    }
  }
}

provider "google" {
  project = var.gcp_project_id
  region = "us-east4"
}