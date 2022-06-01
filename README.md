# Summary
This is a program to compare OVL-q test to other statistical tests.

# How To Use
Edit `config.py` and run `experiment.py` in python to perform the experiment. The result will be written in `output` directory.

Then, run `graph.py` to obtain graphs from the data in `output`. The graphs will be stored in `graphs` directory.

# Details
In `config.py`, the following parameters are defined.

- `num_processes`, `num_trial`: positive integers.
- `sample_sizes`: a list of positive integer.
- `significance_level`: a floating-point number between 0 and 1.
- `x_dist`: a probability density function.
- `y_dists`: a list of probability density functions.

In `experiment.py`, the following steps are performed for each probability density function `y_dist` in `y_dists`.

1. repeat the following steps for each `sample_size` in `sample_sizes`.
   1. repeat the following steps `num_processes` × `num_trial` times.
      1. generate `sample_size` random variables `xs` with the probability functions `x_dist` and `sample_size` random variables `ys` with the probability functions `y_dist`.
      1. subject `xs` and `ys` to the statistical tests (OVL-1, OVL-2, Welch t, F, Mann-Whitney U and Cramér-von Mises).
   1. count how many times the null hypothesis `x_dist` = `y_dist` was rejected with `significance_level`.
1. creace a table with `sample_size` on the vertical axis and the statistical tests on the horizontal axis, and output it as a csv file in `output` directory.
