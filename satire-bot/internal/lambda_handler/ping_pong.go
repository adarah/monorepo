package lambdahandler

import (
	"fmt"

	"github.com/adarah/satirebot/internal/discord"
	"github.com/adarah/satirebot/internal/event"
)

func HandlePingPong(event event.PingPong) {
	fmt.Print("inside the lambda")
	discord := discord.GetSession()
	message := "ping"
	if event.IsPing {
		message = "pong"
	}
	discord.ChannelMessageSend(event.ChannelID, message)
}
