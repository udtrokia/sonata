# Syntax

(sonata (rust lisp))

## Origin

Sonata is just lisp, standing by symbolic expressions, for example:

```lisp
(: "hello, world")
```

### Definition

The definition in sonata is starting with `$`:

```lisp
(foo "hello, world")
```

### Data Type

List is the only fundamental data type in Sonata, we can define a list just like this:

```lisp
(1 2 3)
```

### Operators

Arithmetic operators are the same we using in our caculator:

```lisp
(+ 1 1)
```

### Function

Original sonata just has a single lambda function:

```lisp
((arg) (+ arg 1))
```

### Condition

We can control the flow like this:

```lisp
((= 1 1) (+ 1 1)
 (!= 1 1) (- 1 1))
```