module github.com/adarah/satirebot

go 1.18

// +heroku goVersion go1.18
// +heroku install ./cmd/event_notifier

require (
	github.com/aws/aws-lambda-go v1.29.0
	github.com/aws/aws-sdk-go-v2 v1.16.2
	github.com/aws/aws-sdk-go-v2/config v1.15.3
	github.com/aws/aws-sdk-go-v2/service/lambda v1.21.1
	github.com/bwmarrin/discordgo v0.24.0
	github.com/kelseyhightower/envconfig v1.4.0
)

require (
	github.com/aws/aws-sdk-go-v2/credentials v1.11.2 // indirect
	github.com/aws/aws-sdk-go-v2/feature/ec2/imds v1.12.3 // indirect
	github.com/aws/aws-sdk-go-v2/internal/configsources v1.1.9 // indirect
	github.com/aws/aws-sdk-go-v2/internal/endpoints/v2 v2.4.3 // indirect
	github.com/aws/aws-sdk-go-v2/internal/ini v1.3.10 // indirect
	github.com/aws/aws-sdk-go-v2/service/internal/presigned-url v1.9.3 // indirect
	github.com/aws/aws-sdk-go-v2/service/sso v1.11.3 // indirect
	github.com/aws/aws-sdk-go-v2/service/sts v1.16.3 // indirect
	github.com/aws/smithy-go v1.11.2 // indirect
	github.com/gorilla/websocket v1.4.2 // indirect
	github.com/jmespath/go-jmespath v0.4.0 // indirect
	golang.org/x/crypto v0.0.0-20210421170649-83a5a9bb288b // indirect
	golang.org/x/sys v0.0.0-20201119102817-f84b799fce68 // indirect
)
