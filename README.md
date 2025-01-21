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
mapping: expression
for_if_clause
 | 'for' pattern 'in' sequence ('if' expression)*
pattern: name (,name)*
```

Notice that I didn't implement the "one or more" `for_if_clause` in this tutorial because it is more complicated.

### The comprehension 
```
comp: mapping for_if_clause+
```
Examples:
- ... for_if_clause
- ... for_if_clause for_if_clause ...

where the `mapping` component is

```
mapping: expression
```
Examples:
- x * 2
- x + 1

### The `for_if_clause` component
```
for_if_clause
 | 'for' pattern 'in' sequence ('if' expression)*
```
Examples
- for ... in ...
- for ... in ... if ...
- for ... in ... if ... if ...

where the `pattern` component is 

```
pattern: name (,name)*
```
Examples:
- a
- a, b

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
IntoIterator::into_iter(<sequence>)
  .flat_map(|<pattern>| {
    (true (&& <expression>)*).then(|| <mapping>)
  })
```

## Reference
[Comprehending Proc Macros](https://www.youtube.com/watch?v=SMCRQj9Hbx8)
