import parser
from .sim import Simulation


def run(config):
    sim = Simulation()
    return sim.run()