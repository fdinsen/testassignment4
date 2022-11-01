Feature: Points

Scenario: Increase points
Given we have a snake game
When the snake moves to a spot with an apple
Then the points go up by one