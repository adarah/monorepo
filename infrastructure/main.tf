module "satire_bot" {
  source   = "./satire_bot"
  region   = "us"
  src_path = "../satire-bot"
  event_emitter_aws_access_key_id = "123"
  event_emitter_aws_secret_access_key = "123"
  satbot_discord_token = "123"
}
