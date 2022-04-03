package main

import (
	"fmt"

	lambdahandler "github.com/adarah/satirebot/internal/lambda_handler"
	"github.com/aws/aws-lambda-go/lambda"
)

func main() {
	fmt.Print("Before lambda start")
	lambda.Start(lambdahandler.HandlePingPong)
}
