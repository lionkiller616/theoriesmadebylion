# Field-Synthesis Energy Harvesting

## Abstract

Design of a passive, layered metamaterial harvester pane that captures ambient electromagnetic noise (50 MHz–5 GHz) and converts it to usable DC without active rectifiers. By overlapping multiple resonant cells, embedding nonlinear diodes, and integrating MEMS-based tuners, the system can self-optimize to changing RF environments, enabling battery-free IoT and sensor networks.

## 1. Introduction

Ambient RF energy abounds in urban environments—from broadcast TV and radio towers to Wi-Fi and cellular networks. Conventional harvesters rely on narrowband antennas and active rectifiers, limiting efficiency and requiring external power for tuning. This theory explores a fully passive metamaterial approach:

* **Metamaterial lattice**: engineered resonant cells cover broad spectrum.
* **Nonlinear diodes**: embedded at nodes to perform rectification.
* **Self-tuning MEMS**: mechanical adjustments maintain peak resonance.

## 2. Metamaterial Lattice Design

### 2.1 Unit Cell Structure

Each cell comprises a split-ring resonator (SRR) coupled to a complementary electric resonator (CER). Dimensions are chosen so that the SRR resonates at frequency $f_i$. A lattice stacks cells for frequencies:

$$
  \{f_1, f_2, \dots, f_N\} \subset [50\mathrm{MHz},5\mathrm{GHz}]
$$

Resonance condition for SRR:

$$
  \omega_i = \frac{1}{\sqrt{L_i C_i}}, \quad f_i = \frac{\omega_i}{2\pi}
$$

with $L_i$ and $C_i$ the inductance and capacitance of the $i$-th cell.

### 2.2 Layer Stacking

Layers of unit-cell arrays are laminated with interlayer spacing $d$ to minimize inter-cell coupling except through designed waveguides. The effective surface impedance is tailored via:

$$
  Z_s(\omega) = j\omega \mu_0 d - \frac{j}{\omega \varepsilon_0 d} + Z_{meta}(\omega)
$$

where $Z_{meta}$ is the metamaterial contribution.

## 3. Nonlinear Conversion at Lattice Nodes

### 3.1 Diode-Embedded Nodes

At each SRR gap, a Schottky diode provides rectification. The diode I–V characteristic:

$$
  I_D(V) = I_S \left(e^{\frac{V}{nV_T}} - 1\right)
$$

where $I_S$ is reverse saturation current, $n$ the ideality factor, and $V_T = k_B T/q$.

### 3.2 DC Bus Formation

All diode outputs connect to a common low-loss bus through matching networks. The net harvested power:

$$
  P_{DC} = \sum_{i=1}^N \eta_i P_{RF,i}, \quad \eta_i = \frac{V_{out,i} I_{out,i}}{P_{RF,i}}
$$

where $P_{RF,i}$ is incident power at $f_i$ and \eta\_i the conversion efficiency.

## 4. Self-Tuning Mechanism

### 4.1 MEMS Tuners

Variable capacitors (varactors) on each SRR gap adjust $C_i$ via MEMS actuators, shifting resonance:

$$
  C_i(\alpha) = C_{i,0} + \Delta C_i(\alpha), \quad \alpha\in [0,1]
$$

### 4.2 Feedback Control

A passive RC time-constant network senses harvested voltage amplitude and biases MEMS without active electronics:

$$
  \tau \frac{d\alpha}{dt} + \alpha = f(V_{bus})
$$

where $f(\cdot)$ is a monotonic mapping from bus voltage to actuator position.

## 5. Governing Equations Summary

1. **Resonance**: $\omega_i = 1/\sqrt{L_i C_i}$
2. **Surface Impedance**: $Z_s(\omega) = j\omega \mu_0 d - j/(\omega \varepsilon_0 d) + Z_{meta}(\omega)$
3. **Diode I–V**: $I_D = I_S(e^{V/nV_T}-1)$
4. **Power Conversion**: $P_{DC}=\sum_i \eta_i P_{RF,i}$
5. **Tuning Dynamics**: $\tau d\alpha/dt+\alpha=f(V_{bus})$

## 6. Implementation Pathway

1. **Electromagnetic Simulation**: Use FDTD or FEM (e.g., CST, COMSOL) to optimize SRR/CER geometries.
2. **Microfabrication**: Lithography for PCB-scale prototypes; integrate Schottky diodes and MEMS varactors.
3. **Characterization**: Sweep incident RF in an anechoic chamber; measure DC output vs. frequency.
4. **Optimization**: Iterate lattice parameters, diode matching, and bus impedance.

## 7. Potential Applications

* Battery-free environmental sensors
* Wearable RF harvesters for health monitors
* Smart-building window coatings
* Distributed IoT infrastructure with perpetual power

