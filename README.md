# rust-hacks

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


## Hacks
* teris: Tetris game
* file-handling: Read & write slices to disc
* sdl-window: Open window with SDL, enable OpenGL
* redisload: Redis client
* udp-server: UDP server using Tokio (async)
