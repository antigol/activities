Compile
=======

1. Install [rust and cargo](https://www.rust-lang.org/en-US/downloads.html)
2. Clone this repository and open a terminal in it
3. `cargo build --release`

Execute
=======

`target/release/activities etc/input.txt etc/output.txt 15 ,`

Input and Output file are in csv format. The delimiter is the comma by default.

Input
=====

    m = amount of entries
    n = amount of workshops
    i = 0..m
    j = 0..n


    --------
      VMIN
    --------
      VMAX
    --------
    I|
    D|WISHES
    S|
    --------

    IDS[i]       = name of entry i
    VMIN[j]      = the minimum to place into workshop j
    VMAX[j]      = the maximum to place into workshop j
    WISHES[i][j] = the wish of the entry i to be placed into the workshop j
    VMIN[j] <= VMAX[j]
    0 <= WISHES[i][j] < n


Ouput
=====
The output file contains all the equivalent solutions (with the same score).
There is a pair of column for each solution: one column contains the assigned workshop and the other contain the wish associated with this choice.

     |R|V|R|V|
     |E|A|E|A|...
    I|S|L|S|L|
    D|U|U|U|U|...
    S|L|E|L|E|
     |T|S|T|S|...
     |S| |S| |
     | | | | |...
     |0|0|1|1|


    RESULTS[i] = workshop attributed to entry i
    VALUES[i]  = WISHES[i][RESULT[i]]
    0 <= RESULTS[i] < n

`SCORE = sum over i of VALUES[i]^2`

The program tries to minimize the `SCORE`
The more long time the application is executed, better is the result


Example
=======
Example executed 60 seconds with the comma as separator.

input.txt

      0,1,0,0,5
      5,5,5,5,5
    a,0,1,2,3,4
    b,1,0,2,3,4
    c,3,2,1,0,4
    d,1,2,3,4,0
    e,0,1,2,3,4
    f,4,3,2,1,0
    g,2,3,4,0,1

`./activities input.txt output.txt 60 ","`

output.txt

    a,4,4,0,0,4,4
    b,1,0,1,0,1,0
    c,3,0,4,4,4,4
    d,4,0,4,0,4,0
    e,4,4,4,4,0,0
    f,4,0,4,0,4,0
    g,4,1,4,1,4,1

3 solutions found.
