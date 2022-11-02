Feature: Apple Spawning

Scenario: Apple spawns when no apple is on the screen
Given we have a snake game
When an apple has been eaten
Then an apple should be spawned