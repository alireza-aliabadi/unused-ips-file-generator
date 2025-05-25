## why i got ip ranges start and ends from cli instead of reading user prompt

using while loop has cost of i/o systemcall to wait for user input and unwrapping and parsing costs

but using cli args we don't have systemcall waiting and input wrapping/parsing cost is less by using clap paser and args type structure, also reading from memory has high performance rather than reading user input. in other hand it may cause annoying cli args writing if have many (~255) range but it's rarely usage.
also it costes spliting input and ad storing new arrays (development complexity cost)