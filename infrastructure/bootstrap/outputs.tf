output "tf_state_bucket" {
  value = aws_s3_bucket.tf_state_bucket
}

output "tf_state_lock_table" {
  value = aws_dynamodb_table.tf_state_lock_table
}
