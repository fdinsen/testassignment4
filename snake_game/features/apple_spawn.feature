Feature: Apple Spawning

Scenario: Apple spawns when no apple is on the screen
Given we have a snake game
When the snake moves to a spot with an apple
Then an apple should be spawned