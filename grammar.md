$$
\begin{align}
    [\text{Prog}] &\to \text{fn\space main() [Block]} \\
    [\text{Fn}] &\to 
        \text{fn\space[Ident]\space([Param]*)\space[-> [Ident]]?\space[Block]} \\
    [\text{Param}] &\to
    \begin{cases}
        \text{[Ident]:\space[Ident]}\\
        \text{[[Ident]:\space[Ident],]+[Ident]: [Ident]}
    \end{cases} \\
    [\text{Block}] &\to \text{\{[Expr]*\}} \\
    [\text{Expr}] &\to
    \begin{cases}
        \text{int\_lit} \\
        [\text{Ident}] \\
        \text{let\space[Ident][:[Ident]?]\space=\space[Expr]}; \\
        [\text{Ident}](); \\
        \text{exit([Expr])};
    \end{cases} \\
    [\text{Ident}] &\to 
    \begin{cases}
        \text{\textasciicircum[A-Za-z][A-Za-z0-9\_]*\$} \\
        \text{\textasciicircum[\_][A-Za-z0-9\_]+\$}
    \end{cases}
\end{align}
$$