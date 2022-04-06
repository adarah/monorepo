# Satire Bot
This is a Discord bot written for my Lost Ark guild. The bot is named after the awful name of the guild (we offered to pay for a name change ticket but the Guild leader refused 😔). You can find us in the NA East Una server.

## Architecture
TODO: Add a fancy diagram here.

### Requirements
- The project should be as cheap as possible to run (ideally free!).
- The bot should ideally be reasonably available (otherwise I'd just run it on my own laptop).
- It has to be capable of adding roles to users based on configurable reaction rules.
- Has to be capable of scheduling recurring raid events.

### Technical decisions
#### Webhooks vs websockets
Discord has two primary ways to notify bots that something happened: Webhooks and Websockets.
Using Webhooks would require me to expose an HTTP API, which in turn would need a Load Balancer and an API Gateway. Both of these cost a reasonable amount of money even if they're idle.
Using Websockets however, I can get some free compute hours every month with Heroku worker dynos. This worker is not exposed to the open web, but it can invoke lambdas do the necessary work via the AWS SDK, removing the need for a load balancer and the API gateway.