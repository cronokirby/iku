# Iku (行く)

I like some things about go, but not others, and this is kind of my way of fixing those.

## Sample Code

```
func foo(a I32, b I32) {
  x I32 := 3
  y := 3
  a + b
}

struct S {
  first I32
  second I32
}

impl S {
  func new(a I32) S {
    S { first a, second a }
  }
}

enum E {
  A(I32)
  B
}

func main() {
  e1 := E.A(3)
  e2 := E.B
  s1 := S { first 3, second 4 }
  match e1 {
    E.A(_) => 4
    E.B => s1.second
  }
}
```
