#  ╭──────────────────────────────────────────────────────────╮
#  │ Heroku worker                                            │
#  ╰──────────────────────────────────────────────────────────╯
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

#  ╭──────────────────────────────────────────────────────────╮
#  │ Aws lambdas                                              │
#  ╰──────────────────────────────────────────────────────────╯
resource "aws_iam_role" "assume_role" {
  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Action = "sts:AssumeRole"
        Effect = "Allow"
        Sid    = ""
        Principal = {
          Service = "ec2.amazonaws.com"
        }
      },
    ]
  })
}

data "aws_region" "current" {}

resource "aws_lambda_function" "ping_pong" {
  function_name = "ping_pong"
  role          = aws_iam_role.assume_role.arn
  runtime       = "go1.x"
  filename      = "bazel-bin/satire-bot/ping_pong_handler.zip"
  environment {
    variables = {
      SATBOT_PING_PONG_ARN = aws_lambda_function.ping_pong.arn
      SATBOT_DISCORD_TOKEN = ""
      SATBOT_AWS_REGION    = data.aws_region.current.name
    }
  }
}
