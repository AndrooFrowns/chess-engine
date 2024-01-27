## Chess Engine

In this project I will be working to create a chess engine that 
interacts with the UCI (universal chess interface)
so that it can be played against by a human player.

### Inspiration

I saw the video from suckerpinch 
(https://www.youtube.com/watch?v=DpXy041BIlA&t=422s) 
and thought it seemed like an interesting exercise.

### Progress notes

I decided to use cutechess for the GUI for now. Maybe I'll make my own later,
but it seemed like a good way to start and compiled easily.

Getting the bongcloud opening working was chosen for the inital work because
it's simple and as long as the engine is white it's not preventable

### Goals

* [*] Write code that can submit 1 a4 ... 2 a2 to a UCI compatible GUI.
* [ ] Write a random move chess engine.
* [ ] Write a chess engine that only strives to naively get to a good scoring position
* [ ] Use the score evaluation to write minimax
* [ ] Write alpha beta pruning
* [ ] Abstract the parts written for the previous suggestions to allow for more easily writing custom scoring functions
* [ ] Write an extra couple strategies
* [ ] benchmark parts for speed?
* [ ] more if I still find this interesting to work on.

