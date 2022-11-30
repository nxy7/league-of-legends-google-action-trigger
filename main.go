package main

import "time"

func main() {
	println("Starting up..")
	interval := time.Duration(time.Second * 1)

	// read and apply configuration from trigger.json

	// store it as list of trigger structs(?)

	for {
		// make request to league of legends live server
		println("Loop")

		// loop over triggers and act on all appropriate ones

		// sleep
		time.Sleep(interval)
	}

}
