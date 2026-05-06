# Notes 

```
Notes : cargo.lock
when using cargo build to make sure versions stay same as in the lock file use `cargo build --lock`.  
```
## guessing game

1. By default variables are immutable , thus needed to make the variables mutable via ` mut ` keyword 
2. `.readline()` it is used to take userInput 
3. `&mut guess` used for refrenceing mutable  variable meanning with this refrence data can be changes `&guess` this is read only refrencing 
4. `println!("text{refrence},text{}",refrence+operation)`, here **macros** are used insted of Function via `!`

**What are Macros?**
```
Macros are like functions but it solves the limitation of functions 
1. limited arguments passed on functions , we can pass as mnay arguments we want to in case of macros 
2. in case of funtion it is called , but in case of macro at compile time complete code is replaced  

```