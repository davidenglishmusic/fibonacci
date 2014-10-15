#[test]
fn fibonacci_0_returns_0() {
    if fibonacci(0) != 0 {
        fail!("Zero does not return zero");
    }
}

#[test]
fn fibonacci_1_returns_1() {
    if fibonacci(1) != 1 {
        fail!("One does not return one");
    }
}

#[test]
fn fibonacci_2_returns_1() {
    if fibonacci(2) != 1 {
        fail!("Two does not return one");
    }
}

#[test]
fn fibonacci_3_returns_2() {
    if fibonacci(3) != 2 {
        fail!("Three does not return two");
    }
}

#[test]
fn fibonacci_7_returns_13() {
    if fibonacci(7) != 13 {
        fail!("Seven does not return thirteen");
    }
}

fn fibonacci(x: int) -> int{
  if x <= 1{
    x
  }
  else {
    fibonacci( x - 1) + fibonacci( x - 2 )
  }
}
