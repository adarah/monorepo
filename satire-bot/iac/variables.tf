variable "heroku_region" {
  type        = string
  description = "The target Heroku region. See https://devcenter.heroku.com/articles/regions for more details."
}

variable "aws_region" {
  type        = string
  description = "The AWS region to deploy the bot lambdas in."
}

variable "discord_token" {
  type        = string
  description = "The bot's Discord auth token."
  sensitive   = true
}
