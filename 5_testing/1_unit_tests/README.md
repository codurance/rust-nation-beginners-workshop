# Workshop: Unit Tests

This library contains a function to reverse a string that may or may not work. Write tests for the following test cases to see if it works or not. If you find a problem with the `reverse` function, fix it. The test cases are as follows, on the left is the input and on the right the expected output:

* "robot","tobor"
* "Ramen","nemaR"
* "I'm hungry!","!yrgnuh m'I"
* "",""
* "racecar","racecar"
* "子猫", "猫子"

## Hints

* Remember to create a test module. Otherwise, your test code will be included in the production binary.
* If you get stuck trying to fix `reverse`, check that zero indexing is being used correctly.
* Ranges are inclusive on the left and exclusive on the right. eg, for the range `[1..5]` 1 will be included but 5 will not - `[1,2,3,4]`

## Attribution

Inspired by Exercism's "Reverse a String" exercise
