module "satire_bot" {
  source        = "../satire-bot/"
  heroku_region = "us"
  aws_region    = "us-east-1"
  discord_token = var.discord_token
}
