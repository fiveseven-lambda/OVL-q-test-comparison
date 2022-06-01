from scipy import stats
import math
import gaussian_mixture

num_processes = 32
# num_trial = 3125
num_trial = 625
sample_sizes = [2 ** i for i in range(2, 13)]
significance_level = 0.05

x_dist = stats.norm()
y_dists = [
    [
        stats.norm(0, 1),
        '$\mathrm{Normal}_{0,1}$',
        'N(0,1)',
    ], [
        stats.norm(0.2, 1),
        '$\mathrm{Normal}_{0.2,1}$',
        'N(0.2,1)',
    ], [
        stats.norm(0, 1.1),
        '$\mathrm{Normal}_{0,1.1}$',
        'N(0,1.1^2)',
    ], [
        stats.uniform(-math.sqrt(3), 2 * math.sqrt(3)),
        'Uniform',
        'Uniform',
    ], [
        stats.triang(0.5, -math.sqrt(6), 2 * math.sqrt(6)),
        'Triangular',
        'Triangular',
    ], [
        gaussian_mixture.HalfMixture(stats.norm(-4/5, 3/5), stats.norm(4/5, 3/5)),
        'Mixed',
        'Mixture',
    ], [
        stats.trapezoid((2 - math.sqrt(2)) / 4, (2 + math.sqrt(2)) / 4, -2, 4),
        'Trapezoidal',
        'Trapezoid',
    ],
]
