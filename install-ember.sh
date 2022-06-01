#!/bin/bash

# TODO make sure git, gcc, clang, cmake and make are installed

git clone https://github.com/Lawo/ember-plus.git
cd ember-plus
git checkout v1.8.2.1
mkdir build
cd build
cmake .. && make && sudo make install
