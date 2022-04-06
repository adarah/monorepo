package lambdahandler

import (
	"fmt"

	"github.com/adarah/satirebot/internal/discord"
	"github.com/adarah/satirebot/internal/event"
)

func HandlePingPong(event event.PingPong) error {
	fmt.Print("inside the lambda")
	discord := discord.GetSession()
	message := "ping"
	if event.IsPing {
		message = "pong"
	}
	_, err := discord.ChannelMessageSend(event.ChannelID, message)
	if err != nil {
		return err
	}
	return nil
}
