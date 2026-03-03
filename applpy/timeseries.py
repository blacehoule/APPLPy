"""
Time Series Module

Defines an extension for computing exact distributions for auto-regressive,
    moving average and other stochastic processes

The algorithms implemented in this module were developed by
   Keith Webb and originally implemented in Maple

Procedures:
    1.
"""

from sympy import (
    symbols,
)

x, y, z, t, v = symbols("x y z t v")

"""
    A Probability Progamming Language (APPL) -- Python Edition
    Copyright (C) 2001,2002,2008,2010,2014 Andrew Glen, Larry
    Leemis, Diane Evans, Matthew Robinson

    Licensed under the Apache License, Version 2.0 (the "License");
    you may not use this file except in compliance with the License.
    You may obtain a copy of the License at

        https://www.apache.org/licenses/LICENSE-2.0

    Unless required by applicable law or agreed to in writing, software
    distributed under the License is distributed on an "AS IS" BASIS,
    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
    See the License for the specific language governing permissions and
    limitations under the License.
"""
