package notifier

import (
	"context"
	"encoding/json"

	"github.com/adarah/satirebot/config"
	"github.com/adarah/satirebot/internal/event"
	"github.com/aws/aws-sdk-go-v2/aws"
	"github.com/aws/aws-sdk-go-v2/service/lambda"
	"github.com/bwmarrin/discordgo"
)

func MessageCreate(client *lambda.Client, conf config.Specification) func(s *discordgo.Session, m *discordgo.MessageCreate) error {
	return func(s *discordgo.Session, m *discordgo.MessageCreate) error {
		// Ignore all messages created by the bot itself
		if m.Author.ID == s.State.User.ID {
			return nil
		}
		if m.Content == "ping" {
			payload, err := json.Marshal(event.PingPong{ChannelID: m.ChannelID, IsPing: true})
			if err != nil {
				return err
			}

			_, err = client.Invoke(context.TODO(), &lambda.InvokeInput{
				FunctionName:   aws.String(conf.PingPongArn),
				InvocationType: "Event",
				Payload:        payload,
			})
			if err != nil {
				return err
			}
		}
		return nil
	}
}
