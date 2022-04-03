package lambdahandler

import (
	"github.com/adarah/satirebot/internal/discord"
	"github.com/adarah/satirebot/internal/event"
)

func HandlePingPong(event event.PingPong) {
	discord := discord.GetSession()
	message := "ping"
	if event.IsPing {
		message = "pong"
	}
	discord.ChannelMessageSend(event.ChannelID, message)
}
