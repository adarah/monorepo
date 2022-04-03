#  ╭──────────────────────────────────────────────────────────╮
#  │ Heroku worker                                            │
#  ╰──────────────────────────────────────────────────────────╯
resource "heroku_app" "satire_bot" {
  name   = "satire-bot"
  region = var.region
  sensitive_config_vars = {
    SATBOT_PING_PONG_ARN  = aws_lambda_function.satire_lambda["ping_pong"].arn
    SATBOT_AWS_REGION     = data.aws_region.current.name
    SATBOT_DISCORD_TOKEN  = var.satbot_discord_token
    AWS_ACCESS_KEY_ID     = var.event_emitter_aws_access_key_id
    AWS_SECRET_ACCESS_KEY = var.event_emitter_aws_secret_access_key
  }
}

resource "heroku_build" "satire_bot" {
  app_id = heroku_app.satire_bot.id
  source {
    path = var.src_path
  }
}

resource "aws_iam_role" "event_emitter" {
  name = "event-emitter"
  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Sid    = "EventEmitterInvokeSatbotLambdas"
        Effect = "Allow",
        Principal = {
          Service = "events.amazonaws.com"
        }
        Action   = "lambda:InvokeFunction"
        Resource = [for l in aws_lambda_function.satire_lambda: l.arn]
      }
    ]
  })
}
output "event_emitter_role" {
  value = aws_iam_role.event_emitter.arn
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

resource "aws_lambda_function" "satire_lambda" {
  for_each         = toset([for f in fileset("${var.src_path}/cmd/lambda", "**/*.go") : dirname(f)])
  function_name    = each.key
  role             = aws_iam_role.assume_role.arn
  runtime          = "go1.x"
  filename         = "../bazel-bin/satire-bot/${each.key}.zip"
  handler          = "ping_pong"
  source_code_hash = filebase64sha256("../bazel-bin/satire-bot/${each.key}.zip")
  environment {
    variables = {
      SATBOT_DISCORD_TOKEN = var.satbot_discord_token
    }
  }
}
