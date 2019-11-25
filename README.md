# OEIS A323809

> More formally, let us call "isolated" a set of unvisited squares which is connected through knight moves, but which cannot be extended to a larger such set by adding a further square. Can there exist at some moment a finite isolated set which the knight cannot reach? (Without the last condition, the square a(2016) would clearly satisfy the condition just before the knight reaches it.)

There cannot be a finite isolated set which the knight cannot reach.

$N(x, y)$ means that there exist a step from $x \rightarrow y$.
$N_{small}(x, y)$ means that $\forall q \neq y: N(x, q) \Rightarrow q > y$.
Every square $s$ has a neighbor $n$ such that $N_{small}(n, s) \land n > s$. For $s$ to be unreachable, $n$ must also be unreachable. As $n > s$ is true, this means that we need an infinite unreachable set $X$ with $\forall p \in X: \exist q: N_{small}(q, p)$

> At move 99999, the least yet unvisited square has number 66048, which is near the border of the visited region. This suggests that the knight will eventually visit every square. Can this be proved or disproved through a counter-example?

The knight will eventually visit every square.

The knight goes in circles around the origin (TODO: prove), meaning that there must not be a wall of $p \in X$. Looking at the smallest $s$ which is not visited, this is impossible (TODO: prove).