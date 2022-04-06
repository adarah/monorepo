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
  managed_policy_arns = ["arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"]
}


resource "aws_lambda_function" "satire_lambdas" {
  for_each = toset([
    for f in fileset("${path.module}/../cmd/lambda", "**/*.go") :
    dirname(f)
  ])
  function_name    = each.key
  role             = aws_iam_role.assume_role.arn
  runtime          = "go1.x"
  filename         = "../../bazel-bin/satire-bot/${each.key}.zip"
  handler          = each.key
  source_code_hash = filebase64sha256("${path.module}/../../bazel-bin/satire-bot/${each.key}.zip")
  environment {
    variables = {
      SATBOT_DISCORD_TOKEN = var.discord_token
    }
  }
}


#  ╭──────────────────────────────────────────────────────────╮
#  │ Event emitter IAM user                                   │
#  ╰──────────────────────────────────────────────────────────╯

resource "aws_iam_user" "event_notifier" {
  name                 = "event-emitter"
  path                 = "/satire-bot/"
  permissions_boundary = "arn:aws:iam::aws:policy/service-role/AWSLambdaRole" // Allows invoking any lambdas
}

resource "aws_iam_user_policy" "event_notifier" {
  user        = aws_iam_user.event_notifier.name
  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Action   = "lambda:InvokeFunction"
        Effect   = "Allow",
        Resource = [for _, l in aws_lambda_function.satire_lambdas: l.arn]
      }
    ]
  })
}

resource "aws_iam_access_key" "event_notifier" {
  user = aws_iam_user.event_notifier.name
}

#  ╭──────────────────────────────────────────────────────────╮
#  │ Heroku worker                                            │
#  ╰──────────────────────────────────────────────────────────╯

resource "heroku_app" "satire_bot" {
  name   = "satire-bot"
  region = var.heroku_region
  sensitive_config_vars = merge({
    SATBOT_DISCORD_TOKEN  = var.discord_token
    AWS_ACCESS_KEY_ID     = aws_iam_access_key.event_notifier.id
    AWS_SECRET_ACCESS_KEY = aws_iam_access_key.event_notifier.secret
    AWS_REGION            = var.aws_region
    },
    {
      for l in aws_lambda_function.satire_lambdas :
      "SATBOT_${upper(l.function_name)}_ARN" => l.arn
    }
  )
}

resource "heroku_build" "satire_bot" {
  app_id = heroku_app.satire_bot.id
  source {
    path = "${path.module}/.."
  }
}
