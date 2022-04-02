#  ╭──────────────────────────────────────────────────────────╮
#  │ Terraform state bucket                                   │
#  ╰──────────────────────────────────────────────────────────╯

resource "aws_s3_bucket" "tf_state_bucket" {
  bucket_prefix = "tf-state"

  lifecycle {
    prevent_destroy = true
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
