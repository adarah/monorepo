variable "region" {
  type        = string
  description = "The target Heroku region. See https://devcenter.heroku.com/articles/regions for more details."
}

variable "src_path" {
  type        = string
  description = "Path to the directory containing satire-bot's go.mod"
}

variable "satbot_discord_token" {
  type        = string
  description = "The auth token for the bot's Discord account."
  sensitive   = true
}


variable "event_emitter_aws_access_key_id" {
  type        = string
  description = "The aws access key id of the event emitter"
  sensitive   = true
}

variable "event_emitter_aws_secret_access_key" {
  type        = string
  description = "The aws secret key of the event emitter"
  sensitive   = true
}