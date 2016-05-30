Use the rust version. The c++ version is old.

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
The output file contains all the results foud with the same score.
The first column contains the id's, then one conlum for the first results, one for the values of
the first results. Then two columns for the second results and so on.

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

input.csv

    10,15,14
    20,16,17
    0,1,2
    2,1,0
    0,2,1
    0,1,2
    2,1,0
    0,2,1

`./activities input.txt output.txt 60 ","`

output.csv

    a,4,4,0,0,4,4
    b,1,0,1,0,1,0
    c,3,0,4,4,4,4
    d,4,0,4,0,4,0
    e,4,4,4,4,0,0
    f,4,0,4,0,4,0
    g,4,1,4,1,4,1

Compile
=======

Install the packages `rustc` and `cargo`.
Then, into the folder `rust`, execute the command `cargo build --release`
