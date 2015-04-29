First Iteration
===============

Theory
------

We'll use a random forest of decision trees.  Each decision tree will look like this:

      (153, 2)
     / Y    N \
    2       (64, 1)
           / Y   N \
          1      (82, 9)
                / Y   N \
               9        ...

First, we find what number is most likely to be represented when a particular pixel is shaded.  That will be the training set.  Next, to classify a pixel, we take some number of decision trees and do as follows:

* Iterate over the decision tree.
* Check the pixel in the picture represented by that node in the decision tree.
* If the pixel is shaded, classify the picture as the digit at that node in the tree.
* Otherwise, keep going through the tree.
* If we don't find anything, use a default digit (I chose 1).

Results
-------

In a few words: way too shitty to use.  The thing was no better than a random guess due to bias in the algorithm, and performance actually declined as the number of decision trees went up.  I was prepared for this to be bad, but not this bad.  (Also, it favors eights and threes heavily, for reasons I can't fully grok yet).

Kaggle Score(s)
---------------

Best:  0.09943
Worst: 0.09014 (Had largest number of decision trees, suggests bias would surpass the utility of the trees)

Versions
--------

v0.0.1, v0.0.2

Second Iteration
================

Theory
------

This time I used K nearest neighbors, though possibly incorrectly.  This time, we trained by making a results picture, where each pixel is replaced by a histogram looking something like this:

                          (etc.) |
    Total number of times      5 |      x
    the pixel was shaded       4 |      x       x
    in the subset of pictures  3 |  x   x x     x   x
    representing a digit       2 |x x   x x     x x x
                               1 |x x x x x   x x x x
                                 ---------------------
                                  0 1 2 3 4 5 6 7 8 9
                                         Digit

Testing was done this way.

* Iterate through all the pictures, and for each picture:
* Iterate through all the pixels, and for each pixel:
* Find the K-nearest neighbors in some radius N from the pixel.
* Take the histograms for each neighbor, and add them up to make a bigger histogram.
* Take the mode of the bigger histogram, and add a one to the count for that digit in our (initially blank) results histogram.
* Classify the picture by taking the mode of the results histogram.

Results
-------

Not terrible (certainly better than luck this time!) but not good enough by the standards of the competition.  The algorithm is heavily biased towards zeroes, twos, threes, and sevens.  Four, five, eight, and nine are not represented at all in the output.

Kaggle Score(s)
---------------

Best:  0.38729
Worst: 0.22629 (Went from radius N = 1 to N = 2; may be that the pictures are so small that even a larger radius by one collects pixels too far out)

Versions
--------

v0.0.3, v0.0.4
