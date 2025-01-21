# comp_macro

Convert python list comprehension syntax into rust code
using procedural macros (functional). For example: 

```rust
comp![ x/y for x,y in mylist if y != 0 ]
```

## General macro concept

```
Frontend (source)
   |
   | syn
   |
   |___Intermediate Representation____
                                      |
                               quote! |
                                      |
                            Backend (target)
```

## Grammar
```
comp: mapping for_if_clause+
```
>>> example 1: ... for_if_clause 
>>> example 2: ... for_if_clause for_if_clause ...

```
mapping: expression
```

```
for_if_clause
 | 'for' pattern 'in' expression ('if' expression)*
```
>>> example 1: for ... in ...
>>> example 2: for ... in ... if ...
>>> example 3: for ... in ... if ... if ...

```
pattern: name (,name)*
```
>>> example 1: a
>>> example 2: a, b

## Rust syntax
In the case of 

```python 
x * 2 for x in xs if x > 0
```

The rust code should be 

```rust
IntoIterator::into_iter(xs)
  .flat_map(|x| {
    (x > 0).then(|| x * 2)
  })
```

or in general

```rust
IntoIterator::into_iter(<expression>)
  .flat_map(|<pattern>| {
    (true (&& <expression>)*).then(|| <mapping>)
  })
```

## Reference
[Comprehending Proc Macros](https://www.youtube.com/watch?v=SMCRQj9Hbx8)
