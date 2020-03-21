from .pop import Population
import random


class Simulation:
    def __init__(self):
        self.population = Population()
        self.delta_t = 1

    def tick(self):
        self.population.forward(self.delta_t)

    def run(self):
        infections, deaths, recoveries = [], [], []
        while self.population.deaths + self.population.recoveries < self.population.infections:
            self.tick()
            infections.append(self.population.infections)
            deaths.append(self.population.deaths)
            recoveries.append(self.population.recoveries)
            print(self.population)
        return infections, deaths, recoveries