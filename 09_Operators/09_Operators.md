# Operators

An operator defines some function that will be performed on the data. The data on which operators work are called operands. Consider the following expression −

7 + 5 = 12

Here, the values 7, 5, and 12 are operands, while + and = are operators.

The major operators in Rust can be classified as:

- Arithmetic
- Bitwise
- Comparison
- Logical
- Bitwise
- Conditional

## Arithmetic Operators

Assume the values in variables a and b are 10 and 5 respectively.

|Sr No |  Operator | Description  | Example
|:----:|:----------|:----|:-----------------|
| 1    | +(Addition)     | returns the sum of the operands|a+b is 15
| 2    | -(Subtraction)     | returns the difference of the values|a-b is 5
| 3    | * (Multiplication)     | returns the product of the values	|a*b is 50
|4|/ (Division)|	performs division operation and returns the quotient	|a / b is 2
|5|% (Modulus)|	performs division operation and returns the remainder	| a % b is 0

**NOTE** : The ++ and -- operators are not supported in Rust.

The sample code is given below

```rust
fn main() {
let num1 = 10 ;
let num2 = 2;
let mut res:i32;
 
res = num1 + num2; 
println!("Sum:    {}    ",res); 

res = num1 - num2; 
println!("Difference: {} ",res) ;

res = num1*num2 ;
println!("Product:  {}  ",res) ;

res = num1/num2 ;
println!("Quotient: {}  ",res);

res = num1%num2 ;
println!("Remainder: {}  ",res); 

}

```

output

```rust
Sum:    12  
Difference: 8
Product:  20  
Quotient: 5  
Remainder: 0

```

## Relational Operators

Relational Operators test or define the kind of relationship between two entities. Relational operators return a Boolean value, i.e., true/ false.

Assume the value of A is 10 and B is 20.

|Sr No |  Operator | Description  | Example
|:----:|:----------|:----|:-----------------|
|1| >	|Greater than |	(A > B) is False|
|2| <	| Lesser than |	(A < B) is True|
|3| >=	|Greater than or equal to|	(A >= B) is False
|4|<= |	Lesser than or equal to	|(A <= B) is True
|5|== |	Equality	|(A == B) is false
|6| !=	|Not equal	|(A != B) is True


sample code as shown 

```rust
fn main() {
let A:i32 = 10;
let B:i32 = 20;

println!("Value of A:{} ",A); 
println!("Value of B : {} ",B);
 
let mut res = A>B ;
println!("A greater than B: {} ",res);

res = A<B ;
println!("A lesser than B: {} ",res)  ;

res = A>=B ;
println!("A greater than or equal to  B: {} ",res);

res = A<=B;
println!("A lesser than or equal to B: {}",res)  ;

res = A==B ;
println!("A is equal to B: {}",res)  ;

res = A!=B  ;
println!("A is not equal to B: {} ",res);
}

```

ouput is shown below 

```rust
Value of A:10 
Value of B : 20 
A greater than B: false 
A lesser than B: true 
A greater than or equal to  B: false 
A lesser than or equal to B: true
A is equal to B: false
A is not equal to B: true 

```

## Logical Operators

Logical Operators are used to combine two or more conditions. Logical operators too return a Boolean value. Assume the value of variable A is 10 and B is 20.

|Sr No |  Operator | Description  | Example
|:----:|:----------|:----|:-----------------|
| 1| && (And) |	The operator returns true only if all the expressions specified return true |	(A > 10 && B > 10) is False
| 2| || (OR) |	The operator returns true if at least one of the expressions specified return true	|(A > 10 || B >10) is True
|3| ! (NOT) |	The operator returns the inverse of the expression’s result. For E.g.: !(>5) returns false	| !(A >10 ) is True


```rust

//sample code

```

## Bitwise Operators

Assume variable A = 2 and B = 3

|Sr No |  Operator | Description  | Example
|:----:|:----------|:----|:-----------------|
|1|& (Bitwise AND) |	It performs a Boolean AND operation on each bit of its integer arguments.|	(A & B) is 2
|2| | (BitWise OR)|	It performs a Boolean OR operation on each bit of its integer arguments.|	(A | B) is 3
|3| ^ (Bitwise XOR) |	It performs a Boolean exclusive OR operation on each bit of its integer arguments. Exclusive OR means that either operand one is true or operand two is true, but not both.	| (A ^ B) is 1
|4| ! (Bitwise Not)|	It is a unary operator and operates by reversing all the bits in the operand.	|(!B) is -4
|5| << (Left Shift)|	It moves all the bits in its first operand to the left by the number of places specified in the second operand. New bits are filled with zeros. Shifting a value left by one position is equivalent to multiplying it by 2, shifting two positions is equivalent to multiplying by 4, and so on.	|(A << 1) is 4
|6| >> (Right Shift) |Binary Right Shift Operator. The left operand’s value is moved right by the number of bits specified by the right operand. |	(A >> 1) is 1
|7| >>> (Right shift with Zero) |	This operator is just like the >> operator, except that the bits shifted in on the left are always zero.|	(A >>> 1) is 1

code sample is shown below

```rust

fn main() {
let a:i32 = 2;   // Bit presentation 10 
let b:i32 = 3;   // Bit presentation 11
 
let mut result:i32; 
     
result = a & b;     
println!("(a & b) => {} ",result);
          
result = a | b;          
println!("(a | b) => {} ",result)  ;
       
result = a ^ b;  
println!("(a ^ b) => {} ",result);
 
result = !b; 
println!("(!b) => {} ",result);

result = a << b; 
println!("(a << b) => {}",result); 

result = a >> b; 
println!("(a >> b) => {}",result);

}

```

output is shown below

```rust
(a & b) => 2
(a | b) => 3
(a ^ b) => 1
(!b) => -4
(a << b) => 16
(a >> b) => 0

```

