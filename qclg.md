## Quantum-Cascade Logic Gates

**Abstract**

Quantum-Cascade Logic Gates (QCLGs) leverage engineered intersubband transitions in semiconductor quantum-cascade structures to encode and manipulate classical or quantum bits at terahertz (THz) frequencies. By cascading multiple quantum wells (QWs) and barriers, QCLGs perform logic operations via resonant tunneling and optical pumping, enabling ultrafast, on-chip computing with minimal moving parts.

---

### 1. Introduction

* **Motivation**: As electronic switching approaches physical limits, QCLGs offer an alternative using quantum-engineered heterostructures to process information at optical/THz speeds.
* **Applications**: High-speed signal processing, secure communications, integrated quantum-classical hybrid computing.

### 2. Core Concept

1. **Quantum-Cascade Structure**: A periodic sequence of QWs and barriers creates a ladder of subbands $|1\rangle, |2\rangle, ..., |N\rangle$ with engineered energy separations $E_{i+1,i}\$.
2. **State Encoding**: Logic states $`0`, `1`$ mapped to population distributions across subbands or to presence/absence of intersubband gain at specified frequencies.
3. **Gate Operation**: Optical or electrical pumping redistributes populations via resonant transitions; tunneling between adjacent wells implements conditional dynamics.
4. **Readout**: Emitted photons at transition frequencies or tunneling current pulses indicate gate output.

### 3. Theoretical Foundations

#### 3.1 Subband Energies

Solve Schrödinger equation in one dimension:

$$
-\frac{\hbar^2}{2m^*} \frac{d^2 \psi_i(z)}{dz^2} + V(z) \psi_i(z) = E_i \psi_i(z),
$$

where $V(z)$ is the conduction band profile and $m^*$ effective mass.

#### 3.2 Rate Equations

Population dynamics in level $i$ governed by:

$$
\frac{d n_i}{dt} = \sum_{j \neq i} (W_{ji} n_j - W_{ij} n_i) + G_i - R_i,
$$

where:

* $W_{ij}$: transition rates (tunneling, phonon, optical)
* $G_i$: injection rate
* $R_i$: relaxation/recombination

#### 3.3 Tunneling Coupling

Tunnel coupling between adjacent wells gives splitting $\Delta E_t = 2\hbar \Omega$, with Rabi frequency $\Omega$ proportional to overlap integral:

$$
\Omega = \frac{1}{\hbar} \int \psi_i^*(z) V_b(z) \psi_{i+1}(z) dz,
$$

where $V_b(z)$ barrier potential.

#### 3.4 Logic Operation Model

Implement NAND gate via population inversion:

* Inputs encoded by pump presence at frequencies $\omega_A, \omega_B$.
* Output transition $\omega_{out}$ occurs only if both input pumps absent (inversion maintained), realizing logical NAND.

Truth table realized by control of $G_i$ terms in rate equations.

### 4. Device Architecture

* **Layer Stack**: III-V semiconductor layers (InGaAs/AlInAs) grown by MBE or MOCVD.
* **Waveguide**: Metal–metal or dielectric-clad ridge guiding TE or TM intersubband modes.
* **Contacts**: Distributed injectors and collectors to apply bias and extract current.
* **Electrodes**: Patterned for localized optical pumping or THz excitation.

### 5. Performance Metrics

* **Switching Speed**: $\sim1/\Gamma_{fast}$ where $\Gamma_{fast}$ is fastest transition rate (10–100 ps).
* **Energy per Operation**: Determined by pump power and bias voltage; target $<1$ pJ per gate.
* **Integration Density**: Monolithic integration on chip with footprints $<100$ µm<sup>2</sup> per gate.
* **Fan-Out**: Cascaded stages possible via shared subband transitions and waveguide coupling.

### 6. Workflow

1. **Simulation**:

   * Solve Schrödinger–Poisson for subband energies and wavefunctions.
   * Compute transition rates and optical gain spectra.
2. **Epitaxial Growth**:

   * Grow heterostructure layers with nanometer precision.
   * Characterize via X-ray diffraction and photoluminescence.
3. **Device Fabrication**:

   * Define waveguides and electrodes via photolithography and etching.
   * Metallize and mount on submount for heat sinking.
4. **Testing**:

   * Apply bias and optical/THz pumps to realize input states.
   * Measure emission spectra and tunneling currents for output.
   * Verify logic functionality against truth tables.
5. **Optimization**:

   * Tweak well/barrier widths to adjust energy separations.
   * Optimize waveguide losses and coupling efficiencies.

### 7. Equations Summary

$$
-\frac{\hbar^2}{2m^*} \frac{d^2 \psi_i}{dz^2} + V(z) \psi_i = E_i \psi_i,
$$

$$
\frac{d n_i}{dt} = \sum_{j} (W_{ji} n_j - W_{ij} n_i) + G_i - R_i,
$$

$$
\Omega = \frac{1}{\hbar} \int \psi_i^*(z) V_b(z) \psi_{i+1}(z) dz,
$$

$$
F_{fast}^{-1} \approx 1/\Gamma_{fast}.
$$

### 8. Future Directions

* Integration with plasmonic antennas for efficient THz pumping.
* Quantum coherent gate operation for qubit manipulations using intersubband superpositions.
* Hybrid electronic–photonic circuits combining QCLGs with CMOS control.
* Temperature-tolerant designs for room-temperature THz logic.

---

*This document outlines the concepts, theoretical models, device designs, and workflows for quantum-cascade logic gates using intersubband transitions for ultrafast on-chip logic.*
