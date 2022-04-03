package main

import (
	lambdahandler "github.com/adarah/satirebot/internal/lambda_handler"
	"github.com/aws/aws-lambda-go/lambda"
)

func main() {
	lambda.Start(lambdahandler.HandlePingPong)
}
