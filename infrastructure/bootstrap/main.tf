#  ╭──────────────────────────────────────────────────────────╮
#  │ Terraform state bucket                                   │
#  ╰──────────────────────────────────────────────────────────╯

resource "aws_s3_bucket" "tf_state_bucket" {
  bucket_prefix = "tf-state"
  force_destroy = true

  lifecycle {
    prevent_destroy = false
  }
}

resource "aws_s3_bucket_acl" "tf_state_bucket" {
  bucket = aws_s3_bucket.tf_state_bucket.id
  acl    = "private"
}

resource "aws_s3_bucket_versioning" "tf_state_bucket" {
  bucket = aws_s3_bucket.tf_state_bucket.id
  versioning_configuration {
    status = "Enabled"
  }
}

#  ╭──────────────────────────────────────────────────────────╮
#  │ Terraform state lock                                     │
#  ╰──────────────────────────────────────────────────────────╯

resource "aws_dynamodb_table" "tf_state_lock_table" {
  name         = "TerraformStateLock"
  billing_mode = "PAY_PER_REQUEST"
  hash_key     = "LockID"
  attribute {
    name = "LockID"
    type = "S"
  }
}


#  ╭──────────────────────────────────────────────────────────╮
#  │ Bazel cache bucket                                       │
#  ╰──────────────────────────────────────────────────────────╯

resource "google_storage_bucket" "bazel_cache_bucket" {
  name                        = "bazel_cache_bucket_adarah"
  location                    = "US-EAST4"
  force_destroy               = true
  uniform_bucket_level_access = true


  lifecycle_rule {
    condition {
      age = 30
    }
    action {
      type = "Delete"
    }
  }
}
