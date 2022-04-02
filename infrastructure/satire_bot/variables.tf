variable "region" {
  type        = string
  description = "The target Heroku region. See https://devcenter.heroku.com/articles/regions for more details."
}

variable "src_path" {
  type        = string
  description = "Path to the directory containing satire-bot's go.mod"
}