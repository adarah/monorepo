package discord

import (
	"log"

	"github.com/bwmarrin/discordgo"
)

// GetSession panics if it fails to connect to discord
func GetSession() *discordgo.Session {
	discordSession, err := discordgo.New("Bot " + "")
	if err != nil {
		log.Fatalf("unable to establish discord session, %v", err)
	}
	return discordSession
}
