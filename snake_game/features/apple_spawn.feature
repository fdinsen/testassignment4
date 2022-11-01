Feature: Apple Spawning

Scenario: Apple spawns when no apple is on the screen
Given we have a snake game
When there is no apple on the screen
Then an apple should be spawned