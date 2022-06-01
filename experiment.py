import struct
import os
import subprocess
# import math
import numpy as np
from numpy import random
from scipy import stats
from multiprocessing import Pool
# import matplotlib.pyplot as plt
import pandas as pd

import config

p_values = {}
for sample_size in config.sample_sizes:
    filename = f'p_values_{sample_size}'
    if not os.path.isfile(filename):
        subprocess.run(['cargo', 'run', '--release', '--', '--size', str(sample_size), '--out', os.getcwd() + '/' + filename],
            cwd = os.path.dirname(__file__) + '/p_values')
    with open(filename, 'rb') as f:
        p_values[sample_size] = [[struct.unpack('>d', f.read(8))[0] for _ in range(2)] for _ in range(sample_size + 1)]
        print(sample_size)

def experiment(seed, x_dist, y_dist, sample_size):
    rng = random.default_rng(seed)
    shape = (sample_size, config.num_trial)
    xs = x_dist.rvs(shape, random_state=rng)
    ys = y_dist.rvs(shape, random_state=rng)
    zs = np.concatenate([xs, ys])
    for row in zs.T:
        assert len(np.unique(row)) == sample_size * 2
    delta = np.cumsum(np.where(zs.argsort(axis = 0) < sample_size, 1, -1), axis = 0)
    delta_max = delta.max(axis = 0)
    delta_min = delta.min(axis = 0)
    ret = []
    # OVL-1 (KS)
    ret.append(sum(p_values[sample_size][sample_size - d][0] < config.significance_level for d in np.stack([delta_max, -delta_min]).max(axis = 0)))
    # OVL-2
    ret.append(sum(p_values[sample_size][sample_size - d][1] < config.significance_level for d in delta_max - delta_min))
    # Welch t
    ret.append(np.count_nonzero(stats.ttest_ind(xs, ys, equal_var=False).pvalue < config.significance_level))
    # F
    xv = stats.tvar(xs)
    yv = stats.tvar(ys)
    ret.append(np.count_nonzero(stats.f.sf(np.stack([xv / yv, yv / xv]).max(axis = 0), dfn = sample_size - 1, dfd = sample_size - 1) < config.significance_level / 2))
    # Mann Whitney U
    ret.append(np.count_nonzero(stats.mannwhitneyu(xs, ys).pvalue < config.significance_level))
    # Cramér–von Mises
    ret.append(sum(stats.cramervonmises_2samp(x, y).pvalue < config.significance_level for x, y in zip(xs.T, ys.T)))
    return ret

seed_sequence = random.SeedSequence(0)

for y_dist, y_dist_title, filename in config.y_dists:
    print(filename)
    data_frame = pd.DataFrame(columns = ['OVL-1', 'OVL-2', 'Welch t', 'F', 'Mann-Whitney U', 'Cramér-von Mises'])
    for sample_size in config.sample_sizes:
        print(sample_size)
        seeds = seed_sequence.spawn(config.num_processes)
        args = ((seed, config.x_dist, y_dist, sample_size) for seed in seeds)
        row = Pool(config.num_processes).starmap(experiment, args)
        data_frame.loc[str(sample_size)] = np.array(row).sum(0)
    print(data_frame)
    data_frame.to_csv('output/' + filename + '.csv')

# for y_dist, y_dist_title, filename in y_dists:
#     print(filename)
#     plt.suptitle(r'$\rm N(0, 1)$ vs. ' + y_dist_title)
#     ax = plt.subplot(211)
#     x = np.arange(-4, 4, .01)
#     ax.plot(x, x_dist.pdf(x))
#     ax.plot(x, y_dist.pdf(x))
#     ax = plt.subplot(212, xscale = 'log')
#     result = []
#     ax.plot(sample_sizes, result[:, 0], marker = 'o', label = 'OVL-1')
#     ax.plot(sample_sizes, result[:, 1], marker = 'o', label = 'OVL-2')
#     ax.plot(sample_sizes, result[:, 2], marker = '+', label = 'Welch t')
#     ax.plot(sample_sizes, result[:, 3], marker = 's', label = 'F')
#     ax.plot(sample_sizes, result[:, 4], marker = 'x', label = 'Mann-Whitney U')
#     ax.plot(sample_sizes, result[:, 5], marker = '^', label = 'Cramér–von Mises')
#     ax.legend()
#     ax.set_xlabel('sample size')
#     ax.set_ylabel('rejection ratio')
#     ax.set_ylim(-0.05, 1.05)
#     plt.savefig('graphs/' + filename + '.eps')
#     # plt.savefig(filename + '.png')
#     plt.close()
# 
