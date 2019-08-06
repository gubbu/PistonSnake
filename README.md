# Piston Snake
it is just a simple snake game. Controls are: wasd. No parallel code written by me in there. The player steers the snake over a 32x14 bitmap and has to avoid walls while collecting red blocks("fruits") to make his snake longer and increase his movement speed. If the fruit spawns inside a wall the player is able to pass through it without causing the gameover condition. The Gameover condition is met, when the player
+ collides with a wall
+ collides with himself
A very bad pseudorandom number generator is used that could result in the fruit spawning in the same position over and over again in the lategame. The map and other constants defined at the beginning of main.rs should be tweaked to result in the optimal experience.
Features that would be cool, if added:
+ sounds
+ score counter rendered on screen(maybe use my primitve 5x5 bitmap fontrender)
+ map selection menu state, select and load a map file (large changes in code required) 