# Decision Making

Decision-making structures require that the programmer specifies one or more conditions to be evaluated or tested by the program, along with a statement or statements to be executed if the condition is determined to be true, and optionally, other statements to be executed if the condition is determined to be false.

Shown below is the general form of a typical decision-making structure found in most of the programming languages −

![decision_making](https://user-images.githubusercontent.com/9062443/46945104-95641680-d091-11e8-81ca-8d4ab406442e.jpg)

| S.No | Statement | Description |
|:-----|:----------|:-------------|
| 1 | *if* statement|  An *if* statement consists of a Boolean expression followed by one or more statements.
| 2 | *if...else* statement|An *if* statement can be followed by an optional *else* statement, which executes when the Boolean expression is false.
|3|*else...if* and nested *if* statement|You can use one *if* or *else if* statement inside another *if* or *else if* statement(s). |
|4|*match* statement |
A *match* statement allows a variable to be tested against a list of values.

## If Statement

The *if…else* construct evaluates a condition before a block of code is executed.

### Syntax

```rust
 if boolean_expression {
   // statement(s) will execute if the boolean expression is true  
}
```

If the Boolean expression evaluates to true, then the block of code inside the if statement will be executed. If the Boolean expression evaluates to false, then the first set of code after the end of the if statement (after the closing curly brace) will be executed.

```rust
 fn main(){
    let  num:i32 = 5;
if num > 0 { 
   println!("number is positive") ;
}
}

```

The above example will print `number is positive` as the condition specified by the if block is true.

## If else statement

An if can be followed by an optional else block. The else block will execute if the Boolean expression tested by the if statement evaluates to false.

### Syntax

```rust
if boolean_expression {  
   // statement(s) will execute if the boolean expression is true
} else {
   // statement(s) will execute if the boolean expression is false  
}

```

### FlowChart

![If_else](
https://www.tutorialspoint.com/typescript/images/if_else_statement.jpg)

The if block guards the conditional expression. The block associated with the if statement is executed if the Boolean expression evaluates to true.

The if block may be followed by an optional else statement. The instruction block associated with the else block is executed if the expression evaluates to false.

### Example: Simple if…else

```rust
fn main() {

let num = 12;
if num % 2==0 {
   println!("Even");
 } else {
   println!("Odd");
 }
}

```

The above example prints whether the value in a variable is even or odd. The if block checks the divisibility of the value by 2 to determine the same. Here is the output of the above code −

```rust
Even
```

## Nested If

The **else…if** ladder is useful to test multiple conditions. Its syntax is given below −

### Syntax

```rust
 if boolean_expression1 { 
   //statements if the expression1 evaluates to true 
} else if boolean_expression2 { 
   //statements if the expression2 evaluates to true 
} else { 
   //statements if both expression1 and expression2 result to false 
}

```

When using if…else…if and else statements, there are a few points to keep in mind.

- An if can have zero or one else's and it must come after any else..if's.

- An if can have zero to many else..if's and they must come before the else.

- Once an else..if succeeds, none of the remaining else..if's or else's will be tested.


### Example: else…if ladder

```rust

fn main() {

let num = 2 ;
if num > 0 { 
   println!("{} is positive",num); 
} else if num < 0 { 
   println!("{} is negative",num); 
} else { 
   println!("{} is neither positive nor negative",num) ;
}

}

```

The snippet displays whether the value is positive, negative or zero.

output

```rust
2 is positive

```

## Match Statement

### Syntax

```rust
 let expressionResult = match variable_expression {
    constant_expr1 => {
      //statements;
   },
    constant_expr2 => {
      //statements;
   },
   _ => {
      //default
   }
};

```

```rust
fn main(){
    let state_code="MH";
    let state = match state_code {
        "MH" => {
             println!("Found match for  MH");
             "Maharashtra"},
        "KL" => "Kerala",
        "KA" => "Karnadaka",
        "GA" => "Goa",
         _ => "Unknown"

    };
    println!("State name is {}",state);
}
```