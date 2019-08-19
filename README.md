# krust
Kafka command line tool written in Rust

# Comparisons
## how does cat work?
- copies stdin to stdout
- `cat [options] [filenames] [-] [filenames]`
1. display contents of file
1. view contents of multiple files
1. create a file by redirecting output
1. cat to more/less

## how does tac work?

## how does head work?
- reads the first 10 lines of a given file(s)
- `head [options] [file(s)]`
## how does tail work?
- reads the last 10 lines of a given file

## kafkacat
- consumer mode
- producer mode
- metadata

### requests
- windows support

## things i want to be easier
- reset offsets to a point in time
- CHECK HELIOS CHANNEL FOR MORE EXAMPLES
- using certs
- no buffered messages (looking at you, Kafkacat)
- no silent failures when asking for more messages than are on the topic

# Useful resources
- https://mattgathu.github.io/writing-cli-app-rust/

# TODO
- need sample stuff
- can only get working with dynamic linking feature, need to statically compile. Or do you want to do that? Feels like you do.
- needs to barf if it can't connect to broker
