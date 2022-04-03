package main

import (
	"context"
	"log"

	"github.com/aws/aws-sdk-go-v2/config"
	"github.com/aws/aws-sdk-go-v2/service/lambda"
	"github.com/bwmarrin/discordgo"
	"github.com/kelseyhightower/envconfig"

	satbotconf "github.com/adarah/satirebot/config"
	"github.com/adarah/satirebot/internal/discord"
	"github.com/adarah/satirebot/internal/notifier"
)

func main() {
	var conf satbotconf.Specification
	err := envconfig.Process("satbot", &conf)
	if err != nil {
		log.Fatalf("unable to load bot config, %v", err)
	}

	awsCfg, err := config.LoadDefaultConfig(context.TODO(), config.WithRegion(conf.AwsRegion))
	if err != nil {
		log.Fatalf("unable to load AWS SDK config, %v", err)
	}
	client := lambda.NewFromConfig(awsCfg)

	session := discord.GetSession()
	session.AddHandler(notifier.MessageCreate(client, conf))
	session.Identify.Intents = discordgo.IntentGuildMessages

	err = session.Open()
	if err != nil {
		log.Fatalf("error opening connection, %v", err)
	}

	session.Close()
}
