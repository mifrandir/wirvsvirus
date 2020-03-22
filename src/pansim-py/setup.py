import setuptools

with open("README.md", "r") as fh:
    long_description = fh.read()

setuptools.setup(
    name="pandemic-simlation-MILTFRA",  # Replace with your own username
    version="0.0.1",
    author="Franz Miltz",
    author_email="dev@miltz.me",
    description="A simulation of a disease under certain circumstances in a certain population.",
    long_description=long_description,
    long_description_content_type="text/markdown",
    url="https://github.com/miltfra/wirvsvirus/src/pandemic-simulation",
    packages=setuptools.find_packages(),
    scripts=['pansim'],
    classifiers=[
        "Programming Language :: Python :: 3",
        "License :: OSI Approved :: MIT License",
        "Operating System :: OS Independent",
    ],
    python_requires='>=3.8',
)
