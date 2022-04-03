#  ╭──────────────────────────────────────────────────────────╮
#  │ Heroku worker                                            │
#  ╰──────────────────────────────────────────────────────────╯
resource "heroku_app" "satire_bot" {
  name   = "satire-bot"
  region = var.region
  sensitive_config_vars = {
    SATBOT_PING_PONG_ARN = aws_lambda_function.ping_pong.arn
    SATBOT_DISCORD_TOKEN = ""
    SATBOT_AWS_REGION    = data.aws_region.current.name
  }
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
          Service = "lambda.amazonaws.com"
        }
      },
    ]
  })
}

resource "aws_iam_role_policy_attachment" "basic" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
  role       = aws_iam_role.assume_role.name
}

data "aws_region" "current" {}

resource "aws_lambda_function" "ping_pong" {
  function_name    = "ping_pong"
  role             = aws_iam_role.assume_role.arn
  runtime          = "go1.x"
  filename         = "../bazel-bin/satire-bot/ping_pong_handler.zip"
  handler          = "ping_pong"
  source_code_hash = filebase64sha256("../bazel-bin/satire-bot/ping_pong_handler.zip")
  environment {
    variables = {
      SATBOT_DISCORD_TOKEN = ""
    }
  }
}
