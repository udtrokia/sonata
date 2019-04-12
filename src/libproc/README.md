# libproc

(sonata (rust lisp))

## The Basic Language

```lisp
; Compute the x'th fibonacci number.
(fib (x)
    (< x 3) 1)
    (+ (fib (- x 1)) (fib (- x 2)))
    )

; This expression will compute the 40th number.
(fib 40)
```

### TokenTree

+ List
  + List / Atom
+ Atom 
  + Ident
  + Punct
  + Literal

[1]: http://llvm.org/docs/tutorial/LangImpl01.html
