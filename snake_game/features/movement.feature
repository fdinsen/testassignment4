Feature: Snake movement

Scenario: Moving the snake up
Given we have a snake game
When the W button is pressed
Then the snake moves up

Scenario: Moving the snake down
Given we have a snake game
When the S button is pressed
Then the snake moves down

Scenario: Moving the snake left
Given we have a snake game
When the A button is pressed
Then the snake moves left

Scenario: Moving the snake right
Given we have a snake game
When the D button is pressed
Then the snake moves right

Scenario: Cannot move opposite direction of current direction
Given we have a snake game
When the snake is moving up and we tap down
Then the snake moves up

Scenario: Wrapping movement
Given we have a snake game
When the snake moves over the edge
Then the snake appears on the opposite side