## if 表达式

* if 表达式允许根据条件来执行不同的代码分支
* 这个条件必须是 bool 类型
* if 表达式中，与条件相关联的代码块就叫分支
* 可选的，可以再后面加 else 


```rust
fn main() {

    let number = 6;
    if number %4 ==0 {
        println!("number is divisible by 4");
    } else if number %3 == 0 {
        println!("number is divisible by 3");
    } else {
        println!("number is not divisible by 4,3");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);
}
```


