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
- status bar on reading from topic

# Useful resources
- https://mattgathu.github.io/writing-cli-app-rust/

# Building
- `clang: error: linker command failed with exit code 1 (use -v to see invocation)`
    - If you see this error involving ZSTD functions TODO
    
# Notes
- The benefits of acquiring a stdout lock don't become apparent until you have multiple processes writing to stdout
- Base consumer is slightly faster than stream consumer, and easier to implement - at least with some simple benchmarking

# TODO
- get rid of unwraps
- need sample stuff
- can only get working with dynamic linking feature, need to statically compile. Or do you want to do that? Feels like you do.
- needs to barf if it can't connect to broker
- fastest way to write to stdout
- baseconsumer vs streamconsumer

# Testing
- cargo build && ./target/debug/krust --broker 127.0.0.1:9092 -t mytopic

