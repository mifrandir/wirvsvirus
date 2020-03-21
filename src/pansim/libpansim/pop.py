# This module contains everything that has to do with a population.
#
# This includes the following:
# - relations between people
# - ages

import numpy
import random
EXAMPLE_AGE_GROUPS = [10 * i for i in range(10)]
EXAMPLE_AGE_D = [0.07, 0.08, 0.1, 0.1, 0.1, 0.2, 0.12, 0.1, 0.08, 0.05]
# https://www.businessinsider.com/coronavirus-death-age-older-people-higher-risk-2020-2?op=1&r=US&IR=T
EXAMPLE_DEATH_D = [
    0.002, 0.002, 0.002, 0.004, 0.013, 0.036, 0.08, 0.148, 0.148, 0.148, 0.148
]
EXAMPLE_CONTAGIOUS_T = 14
EXAMPLE_SICK_T = 14
# https://en.wikipedia.org/wiki/Coronavirus_disease_2019#Prognosis
EXAMPLE_DEATH_T = 8


class Population:
    def __init__(self,
                 size=1000,
                 contagiousness=0.0003,
                 age_groups=EXAMPLE_AGE_GROUPS,
                 age_dist=EXAMPLE_AGE_D,
                 lethality_dist=EXAMPLE_DEATH_D,
                 time_to_kill=EXAMPLE_DEATH_T,
                 time_while_contagious=EXAMPLE_CONTAGIOUS_T,
                 time_while_sick=EXAMPLE_SICK_T,
                 initial=1):
        self.size = size
        self.age_groups = numpy.array(age_groups)
        self.age_dist = numpy.array(age_dist)
        self.relations = numpy.zeros((size, size))
        # TODO: Use numpy functiosn to do this
        for i in range(self.size):
            for j in range(i):
                r = random.gauss(0.5, 0.5 / 3)
                self.relations[i][j] = r
                self.relations[j][i] = r
        self.infected_for = numpy.full(size, -1)
        self.infections = 0
        self.dead = numpy.zeros(size, dtype=numpy.bool)
        self.doomed = numpy.zeros(size, dtype=numpy.bool)
        self.deaths = 0
        self.recoveries = 0
        self.recovered = numpy.zeros(size, dtype=numpy.bool)
        self.ages = numpy.array(self.__get_ages())
        if len(lethality_dist) <= len(self.age_groups):
            raise ValueError(
                f"The given lethality distribution is not longer than the given age groups. (expected: >{len(self.age_groups)}, actual: ={len(lethality_dist)}"
            )
        self.lethality_dist = lethality_dist
        self.time_to_kill = time_to_kill
        self.contagiousness = contagiousness
        self.time_while_contagious = time_while_contagious
        self.time_while_sick = time_while_sick
        for _ in range(initial):
            i = int(random.random() * self.size)
            while self.is_infected(i):
                i = int(random.random() * self.size)
            self.infect(i)

    def check_size(self, a):
        if a < 0:
            raise ValueError(f"The given index ({a}) is less than zero.")
        if a >= self.size:
            raise ValueError(
                f"The given index ({a}) is greater than or equal to the size ({self.size}) of the population."
            )

    def check_sizes(self, l):
        for a in l:
            self.check_size(a)

    # Returns the relation between a and b
    def rel(self, a, b):
        self.check_sizes([a, b])
        return self.relations[a, b]

    # Returns the minimum age of person a, i.e. the lower boundary if their age group.
    def get_age(self, a):
        self.check_size(a)
        return ([
            0,
        ] + +self.age_groups)[self.ages[a]]

    # Returns the age group person a is in.
    def __get_ages(self):
        ages = []
        group = -1
        left = 0
        for i in range(self.size):
            if left == 0:
                group += 1
                left = self.size * self.age_groups[i]
            ages.append(group)
            left -= 1
        return ages

    def contagious_set(self):
        # TODO: Optimize
        s = set()
        for i in range(self.size):
            if self.is_contagious(i):
                s.add(i)
        return s

    def is_alive(self, a):
        self.check_size(a)
        return not self.dead[a]

    def is_infected(self, a):
        return self.infected_for[a] >= 0

    def is_contagious(self, a):
        return 0 < self.infected_for[a] <= self.time_while_contagious

    def forward(self, delta_t):
        # Simulate encounters
        # TODO: Decrease number of random values elegantly.
        enc_ps = numpy.random.uniform(size=(self.size, self.size),
                                      high=delta_t)
        for i in range(self.size):
            for j in range(i):
                if enc_ps[i][j] < self.relations[i][j]:
                    self.encounter(i, j, delta_t=delta_t)
        for i in range(self.size):
            # Are you lucky?
            if self.infected_for[i] < 0:
                continue
            # Are you very unlucky?
            if self.dead[i]:
                continue
            self.infected_for[i] += delta_t
            # Are you dead yet?
            if self.doomed[i]:
                di = self.time_to_kill - self.infected_for[i]
                if di <= delta_t:
                    self.dead[i] = 1
                    self.deaths += 1
            # Maybe it's over for you?
            # TODO: Optimize this condition.
            elif not self.recovered[i] and self.infected_for[
                    i] - delta_t <= self.time_while_sick < self.infected_for[i]:
                self.recoveries += 1
                self.recovered[i] = 1

    def encounter(self, a, b, delta_t=1):
        if self.is_infected(a) == self.is_infected(b):
            # If both are infected, they can't be re-infected.
            # If neither are infected, they can't infect each other.
            return
        if not self.is_contagious(a) and not self.is_contagious(b):
            # If neither of them is infected, they can't infect each other.
            return
        if random.random() >= self.contagiousness * delta_t:
            # This was a very lucky encounter...
            return
        self.infect(a)
        self.infect(b)

    def infect(self, a):
        if self.is_infected(a):
            return
        self.infections += 1
        self.infected_for[a] = 0
        lethality_p = self.lethality_dist[self.ages[a]]
        if random.random() < lethality_p:
            self.doomed[a] = 1

    def __str__(self):
        return f"{self.size} {self.infections} {self.deaths} {self.recoveries}"
