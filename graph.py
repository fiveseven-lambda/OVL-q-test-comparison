import pandas as pd
import numpy as np
import os

import config
import matplotlib.pyplot as plt

for y_dist, y_dist_title, filename in config.y_dists:
    data_frame = pd.read_csv('output/' + filename + '.csv', index_col=0)
    data_frame /= config.num_processes * config.num_trial
#    plt.suptitle(r'$dfdfdf\rm N(0, 1)$ vs. ' + y_dist_title)
    ax = plt.subplot(211)
    x = np.arange(-4, 4, .01)
    ax.plot(x, config.x_dist.pdf(x), label='$\mathrm{Normal}_{0,1}$')
    ax.plot(x, y_dist.pdf(x), label=y_dist_title)
    ax.set_ylabel('probability density')
    ax.legend()
    ax = plt.subplot(212, xscale = 'log')
    print(data_frame)
    data_frame.plot(ax=ax, style=['o-', 'o-', '+-', 's-', 'x-', '^-'])
    ax.set_xlabel('sample size (n)')
    ax.set_ylabel('statistical power')
    ax.set_ylim(-0.05, 1.05)
    plt.tight_layout()
    plt.savefig('graphs/' + filename + '.eps')
    plt.savefig('graphs/' + filename + '.png')
    plt.close()
#    os.system('open graphs/' + filename + '.eps')
