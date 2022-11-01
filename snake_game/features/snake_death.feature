Feature: Snake Death

Scenario: Snake does not die when moving to free spot
Given we have a snake game
When the snake moves to a free spot
Then it does not die

Scenario: Snake dies when hitting itself
Given we have a snake game
When the snake moves to a spot that is already occupied by the snake
Then it dies