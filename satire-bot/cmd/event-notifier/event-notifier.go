package main

import "github.com/adarah/satirebot/internal"

func main() {
	greeter := internal.NewGreeter()
	greeter.Greet()
}
