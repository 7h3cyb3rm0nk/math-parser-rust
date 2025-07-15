```
Expression  := Term ( ("+" | "-") Term )*
Term        := Factor ( ("*" | "/") Factor )*
Factor      := Unary | Unary "^" Factor
Unary       := ("-")* Primary
Primary     := NUMBER | "(" Expression ")"

```
