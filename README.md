# Roman to Arabic kata

Learning exercise, writing a code kata to convert roman numerals to arabic numbers. This is simply a reversal of the kata at: <https://codingdojo.org/kata/RomanNumerals/>

The Romans wrote numbers using letters : I, V, X, L, C, D, M. (notice these letters have lots of straight lines and are hence easy to hack into stone tablets)
Part I

The Kata says you should write a function to convert from Roman Numerals to "normal" Arabic numbers: eg

     null --> 0
     I --> 1
     III --> 3
     IV --> 4
     V --> 5
     VI --> 6
     VII --> 7
     VIII --> 8
     IX --> 9
     X --> 10
     XXX --> 30
     XL --> 40
     L --> 50
     C --> 100
     CCC --> 300
     CD --> 400
     D --> 500
     CM --> 900
     M --> 1000
     MCCC --> 1300
     MD --> 1500
     MDCCCXXXII --> 1832
    etc.

For a full description of how it works, take a look at [this useful reference website](http://www.novaroma.org/via_romana/numbers.html).

There is no need to be able to convert numbers larger than 3999. (The Romans themselves didn’t tend to go any higher.)