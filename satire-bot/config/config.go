package config

type Specification struct {
	AwsRegion    string `split_words:"true"`
	PingPongArn  string `split_words:"true"`
	DiscordToken string `split_words:"true"`
}
