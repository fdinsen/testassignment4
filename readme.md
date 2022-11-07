# Test Assignemnt 4
## 1. Mockito Powerups
**Answer the following questions about Mockito. Use code examples in your explanations.**

* **How do you verify that a mock was called?**
  * Using the verify() function.
  * ![Verify example](images/mockito-verify.png)
* **How do you verify that a mock was NOT called?**
  * Pass the never() function as a second argument to the verify() function.
  * ![Never example](images/mockito-never.png)
* **How do you specify how many times a mock should have been called?**
  * Using the VerificationModeFactory.times() function. Pass the amount of times the mock should have been called.
  * ![Times example](images/mockito-times.png)
* **How do you verify that a mock was called with specific arguments?**
  * Using the verify() function. Pass the argument that you want to verify was passed.
  * ![Verify example](images/mockito-verify.png)
* **How do you use a predicate to verify the properties of the arguments given to a call to the mock?**
  * Use the argThat() function, and pass it a lambda function that compares the properties of the arguments.
  * ![ArgThat example](images/mockito-argthat.png)

## 2. Snake Game
Snake game has been implemented in Rust. You can play it either by running the snake_game.exe, or if you prefer to compile it yourself:
1. Install [Rustup](https://www.rust-lang.org/tools/install) (contains everything needed for Rust, including compiler and the package manager Cargo)
2. Navigate to snake_game directory and run the following command:
   1. cargo run

![Snake Game Example](images/snake.png)