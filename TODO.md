### Day 1
Day one we were successfully able to connect the client and server written in
rust and sent up some custom sonic pi code to make sounds come out from the
ras-pi.

#### Day 1 First Try
I was bamboozled by a viewer with whom shall not be named and my victory, which
was sure, was defeated before my eyes.

#### Learnings
`Arc` types can be used to share data amoung threads by calling clone.  This
will create a new incremented atomic reference increment.  This was pretty
freaking awesome.

This allowed my options to be shared amoung the closures for each incoming websocket.

##### Follow Ups
`Box` + `Leak` was mentioned as an alternative because I don't technically ever
mutate the options, therefore since its read only and its lifetime is the
lifetime of the application, I could just leak a bunch of boxed references.  I
get the concept, but I don't understand how to do it in rust.

#### Basic Setup
Client: A rust ws client from my computer.
Server: A rust ws server from the raspi
Sonic Pi: The music program
sonic_pi: The command line utility to update sonic pi.

### Day 2
Get twitch's chat hooked up to my program.  Get quirk.gg hooked up for events.

