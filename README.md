Small project that allows controlling home automation and uses League of Legends events as triggers. To do that it monitors League of Legends live server (LoL actually spins up webserver during game so apps like blitz/porofessor can access game data) and based on content of trigger.json sends requests to specified address.

It's designed to work with home assistant which seems to be the most reliable smart home automation tool.

My language of choice for this project is rust as it already has great crate covering LoL Live Game API so I don't need to serialize responses myself.