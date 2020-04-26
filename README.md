# rust-hacks

## Hacks
* threadpool: Using a channel between threads
* teris: Tetris game
* file-handling: Read & write slices to disc
* sdl-window: Open window with SDL, enable OpenGL
* redisload: Redis client
* udp-server: UDP server using Tokio (async)

## Preparations
* brew install sdl2
* brew install SDL2_image

### Linux: make sure the linuxbrew libraries are found
```
export LIBRARY_PATH=/home/linuxbrew/.linuxbrew/lib
export LD_LIBRARY_PATH=/home/linuxbrew/.linuxbrew/lib
```

## Run
Use nightly due to current status of async
`cargo +nightly run`

## Other
```
# Get latest
rustup update

# Add new binary
cargo new <name>

# Add new lib
cargo new --lib <name>
```

## Update rust-analyzer
```
git clone https://github.com/rust-analyzer/rust-analyzer.git && cd rust-analyzer
cargo xtask install --server
```
