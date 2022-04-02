resource "heroku_app" "satire_bot" {
  name   = "satire-bot"
  region = var.region
}

resource "heroku_build" "satire_bot" {
  app_id = heroku_app.satire_bot.id
  source {
    path = var.src_path
  }
}