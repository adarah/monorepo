module "satire_bot" {
  source        = "../satire-bot/iac"
  heroku_region = "us"
  aws_region    = "us-east-1"
}
