| Command | Functionality                                                            |
|:-------:| ------------------------------------------------------------------------ |
|    i    | increment accumulator                                                    |
|I|Increase all elements in stack by one|
|    d    | decrement accumulator                                                    |
|    s    | square accumulator                                                       |
|    S    | square root accumulator if >0 else ...                                   |
|    o    | print decimal value of accumulator                                       |
|    c    | print ascii value of accumulator, if it's ascii                          |
|    C    | transfer accum val to counter                                            |
|    n    | set the accumulator as the next prime                                    |
|    p    | set the accumulator as previous prime                                    |
|    v    | push to stack                                                            |
|    ^    | pop from stack                                                           |
|    r    | reset accumulator to 0                                                   |
|    R    | reset accum to 1                                                         |
|    f    | set accumulator to funny number 69                                       |
|    b    | branching based on bool flag, jump next_command distance if F            |
|    T    | set bool flag as True                                                    |
|    F    | set bool flag as False                                                   |
|    L    | jump back accumulator steps, until counter is 0 (decs counter by itself) |
|    l    | same as L, but doesn't decrease counter                                  |
|    w    | overwrite data at n..m index with the next m-n value                     |
|    m    | does mod of accumulator by counter                                       |
|    e    | check is even (sets bool flag)                                           |
|    k    | skip input instructions                                                  |
|    K    | go back input instruction                                                |
|    .    | take input from stdin                                                    |
|   ' '   | print whitespace                                                         |
|    _    | new line                                                                 |

Examples
--------
'adarsh': iiisisdddciiicdddcnnnnnicicppppic

Print O or E if number is odd or even
-------------------------------------
.eb5fck9fnnnc

Print numbers from 1 to infinity
--------------------------------
rio K3

print P if prime, else print N
--------



