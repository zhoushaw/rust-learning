## 循环

存在三种循环：
* loop
* for
* while

## loop
loop 必须手动终止否则会一直执行

* 命令行执行后通过 ctrol + c
* 通过 break 语句终止


```rust
fn main(){
    loop{
        println!("msg!");
    }

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    }
    println!("The result is: {}",result);
}
```


## while 条件循环

每次执行循环体之前都判断一次条件

```rust
fn main(){
    let mut number = 3;
    while number != 0 {
        println!("{}!",number);
        number = number -1;
    }
    println!("LIFTOFF!!!");
}
```


## for 循环遍历集合

```rust
fn main(){
    let a = [10,20,30,40,50];
    for element in a.iter {
        println!("the value is: {}",element);
    }
}
```

## range

* 标准库提供
* 指定一个开始数字和一个结束数字，Range 可以生成它们之间的数字（不含结束）
* rev 方法可以反转 Range


```rust
fn main() {
    for number in (1..4).rev() {
        println!("{}!",number);
    }
    println("LIFTOFF!");
}
```


