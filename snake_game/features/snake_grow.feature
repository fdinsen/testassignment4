Feature: Snake growth

Scenario: Snake does not grow when moving to free spot
Given we have a snake game
When the snake moves to a free spot
Then the snake does not grow

Scenario: Snake grows when eating apple
Given we have a snake game
When the snake moves to a spot with an apple
Then the snake grows by one