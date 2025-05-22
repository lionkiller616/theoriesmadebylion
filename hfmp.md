# High-Frequency Magnetohydrodynamic Pumps: 
**Liquid Metal Flow Without Moving Parts**

---

## 1. Introduction

Magnetohydrodynamic (MHD) pumps drive electrically conductive fluids—such as liquid metals—by Lorentz forces generated from crossed electric and magnetic fields, eliminating mechanical moving parts. Operating at high-frequency alternating currents allows precise control of flow rates, reduced electrode polarization, and minimized electrochemical degradation, making them ideal for advanced cooling systems and metal processing.

**Aim:** Present the concept, theoretical foundation, governing equations, logical workflow, and practical considerations for high-frequency MHD pumps using liquid metal.

---

## 2. Conceptual Overview

1. **MHD Principle:**

   * Lorentz force: $\mathbf{F} = \mathbf{J} \times \mathbf{B}$, where $\mathbf{J}$ is current density and $\mathbf{B}$ is magnetic flux density.
   * In a channel, this body force drives fluid motion without moving components.

2. **High-Frequency Excitation:**

   * Apply alternating current (AC) at frequency $f$ to electrodes in the channel walls.
   * Use static or time-varying magnetic fields (permanent magnets or AC coils).
   * High $f$ (kHz–MHz) reduces concentration polarization and bubble formation at electrodes.

3. **Applications:**

   * Liquid-metal cooling in nuclear reactors or high-power electronics.
   * Metal casting, electrochemical synthesis, and flow metering.

---

## 3. Theoretical Framework

### 3.1 Governing Equations

1. **Navier–Stokes with Lorentz Force:**

   $$
   \rho \left(\frac{\partial \mathbf{u}}{\partial t} + \mathbf{u}\cdot\nabla\mathbf{u}\right) = -\nabla p + \mu \nabla^2 \mathbf{u} + \mathbf{J}\times\mathbf{B},
   $$

   where:

   * $\rho$: fluid density; $\mathbf{u}$: velocity; $p$: pressure; $\mu$: dynamic viscosity.

2. **Ohm’s Law for Moving Conductor:**

   $$
   \mathbf{J} = \sigma (\mathbf{E} + \mathbf{u}\times\mathbf{B}),
   $$

   where $\sigma$ is electrical conductivity, and $\mathbf{E}$ is electric field.

3. **Maxwell–Faraday (Quasistatic Approximation):**

   $$
   \nabla \times \mathbf{E} = -\frac{\partial \mathbf{B}}{\partial t}, \quad \nabla \times \mathbf{B} = \mu_0 \mathbf{J},
   $$

   for frequencies below electromagnetic wave propagation regimes (skin depth large compared to channel dimensions).

### 3.2 Lorentz Force in Channel

For a rectangular duct of height $h$ with uniform $B_z$ and electrode-driven current density $J_x(t) = J_0 \cos(2\pi f t)$:

$$
F_y(t) = J_x(t) B_z = J_0 B_z \cos(2\pi f t).
$$

Time-averaged body force (if symmetric) can drive net flow when using phase-shifted currents or spatially varying fields.

### 3.3 Skin Depth and Frequency Limit

Electromagnetic skin depth in liquid metal:

$$
\delta = \sqrt{\frac{2}{\mu_0 \sigma 2\pi f}},
$$

with $\mu_0$ vacuum permeability. To ensure uniform current distribution, choose $f$ such that $\delta \gg$ duct thickness.

---

## 4. Equations & Logic Flow

1. **Channel & Field Design:**

   * Select duct dimensions $(w,h,L)$ and magnet arrangement to produce desired $B_z$.

2. **Electrode Configuration:**

   * Pattern electrodes on opposing walls; drive AC source with controlled amplitude $J_0$ and frequency $f$.

3. **Compute Lorentz Force Density:**

   * $\mathbf{f}(t) = J_0 B_z \cos(2\pi f t)\hat{y}.$

4. **Solve Flow Field:**

   * Numerically integrate Navier–Stokes with oscillatory body force or use analytical solutions for laminar duct flow under body force.

5. **Assess Time-Averaged Velocity:**

   * For sinusoidal forcing, average over $T=1/f$:

   $$
   \bar{u}(y) = \frac{1}{T} \int_0^T u(y,t) \,dt.
   $$

6. **Optimize Frequency:**

   * Ensure $f$ maximizes net flow while minimizing polarization and joule heating.

---

## 5. Practical Implementation Notes

* **Liquid Metal Selection:**

  * Galinstan (Ga–In–Sn), LBE (lead–bismuth eutectic), or liquid sodium for high conductivity.
* **Magnet System:**

  * NdFeB permanent magnets for static fields or copper coils for tunable $B$.
* **Electrode Material:**

  * Corrosion-resistant metals (e.g., platinum, stainless steel) or coated with inert layers.
* **Thermal Management:**

  * Account for joule heating; integrate cooling channels if necessary.
* **Control Electronics:**

  * High-frequency AC drivers with phase control; monitoring of current and voltage to prevent arcing.

---

## 6. Conclusion

High-frequency MHD pumps enable contactless, precise control of liquid metal flows by exploiting Lorentz forces from AC-driven currents in magnetic fields. Proper tuning of frequency, electrode configuration, and channel geometry yields efficient, reliable pumping for advanced thermal management and metallurgical processes without moving components.

---

*End of Document*
