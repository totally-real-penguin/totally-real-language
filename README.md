# Totally Real Language

An attempt to make a lanuage in rust it's not 
realy going to be very good but its just for fun
and not meant to be a good language since I can
barely code in one there is no chance I could
make one.


### About the language

I intend for the language to be a cross between rust
and gdscript. (The language of the Godot game engine)
as I like features from both languages and would love
to fuse them like frankenstein stitching his monster
together from different bodies. It doesn't really have
any defining features planned that will seperate it from
other languages except it's lack of features


## Part 1 The lexer:

I have started work on the lexer and it is nearly done
but after the lexer is the actually difficult part, I think.
I have to make an Abstract syntax tree (AST) / Action tree.
What the lexer does, to my understanding, is read in string
and split it up into keywords and symbols called tokens.
These tokens are needed later on so that we can actually evaluate
when to do what. Some of (Really most of) my implementation is bad
but it can be polished later if I ever want to improve it. As stated
before this is how I am learning rust. The thought proccess behind this
is that if you were thrown into water without the ability to swim
you would atleast learn how to keep yourself afloat or you will
drown (which is the possibility I don't want to think about)


### Part 1.5 Thoughts on implementing the AST
___

An AST just takes the tokens and constructs them into an ordered
list of things todo so we can run all the steps the right way round.
For a demonstration of what it would do you can think of BIDMAS
(Brackets, Indices, Division/Multiplication, Addition/Subtraction)
if you had the expression 4 - (5 + 3) / 2 you can split it up
by the order you would solve

```
    -
 4     /
     +   2
    5 3
```
Reading the operations deepest in the tree first until reaching
the top. Here it reads 5+3 then Ans/2 and then 4-Ans.
Although I don't fully know how to implement everything yet as
it gets a bit complicated with how functions, loops and if statements
or atleast I think it will.

