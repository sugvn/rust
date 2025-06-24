a good advice i found for visualising and implementing recursive functions

"Don't try to descend the stack in your head. That way madness lies. By the magic of mathematic induction, every recursive invocation below you can be assumed to be debugged and already working thanks to the programmer who is the future you. Treat it like a call to a library function that you don't have the source code to. If it helps you to work it out, comment it out temporarily and replace it with its results for a test case of reasonable size.

In other words, you don't need to be able to follow the complete algorithm. All you need to know is how to make the problem slightly smaller so you can call an algorithm that already works thanks to recursive you, and write a trivial base case. I can't stress that enough. Trust that the recursive call already works. Once you do that, recursive algorithms become way easier."
