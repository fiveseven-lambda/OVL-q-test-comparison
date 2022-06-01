import numpy as np

class HalfMixture:
    def __init__(self, dist1, dist2):
        self.dist1 = dist1
        self.dist2 = dist2
    def rvs(self, shape, random_state):
        choice = random_state.integers(2, size=shape)
        ret = np.zeros(shape)
        index = np.nonzero(choice)
        ret[index] = self.dist1.rvs(np.count_nonzero(choice), random_state=random_state)
        choice = 1 - choice
        index = np.nonzero(choice)
        ret[index] = self.dist2.rvs(np.count_nonzero(choice), random_state=random_state)
        return ret
    def pdf(self, x):
        return (self.dist1.pdf(x) + self.dist2.pdf(x)) / 2

# from numpy import random
# from scipy import stats
# import matplotlib.pyplot as plt
# 
# rng = random.default_rng(0)
# shape = (1000, 1000)
# x = HalfMixture(stats.norm(-5, 1), stats.norm(5, 1)).rvs(shape, random_state=rng)
# plt.hist(np.ravel(x), bins = 500)
# plt.show()