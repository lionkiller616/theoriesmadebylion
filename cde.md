# Coherence-Distance Equation

## Abstract

The Coherence-Distance Equation quantifies how the maximum separation of entangled quantum systems grows in an expanding universe. It is given by:

$$
D_c(t) = \lambda_c \exp\biggl(\frac{H_0 t}{\tau_q}\biggr),
$$

where:

* $D_c(t)$: maximum coherent separation at cosmic time $t$.
* $\lambda_c$: initial coherence length at $t=0$.
* $H_0$: Hubble constant (current expansion rate).
* $\tau_q$: intrinsic quantum coherence time scale.

This relation predicts scalability limits for quantum networks over cosmological distances.

## 1. Premise & Motivation

Entangled particles maintain correlations only while coherence persists. In an expanding spacetime, separation increases with cosmic expansion. Qubits separated by distance greater than $D_c(t)$ decohere due to spacetime-induced dephasing.

* **Initial coherence**: $\lambda_c$ is set by system design (e.g., laboratory entangled photons).
* **Expansion factor**: cosmic scale factor yields exponential growth in separation relative to coherence time.

## 2. Theoretical Derivation

### 2.1. Spacetime Expansion Model

Friedmann–Lemaître–Robertson–Walker (FLRW) metric gives scale factor $a(t)\approx e^{H_0 t}$ for $\Lambda$-dominated era.

### 2.2. Coherence Decay

Quantum coherence decays over time as

$$
C(t) = C_0 e^{-t/\tau_q},
$$

with $C_0$ initial coherence amplitude.

### 2.3. Combining Expansion and Coherence

A pair separated by physical distance $d(t)=d_0 a(t)$. Setting $d(t)=D_c(t)$ when coherence falls below threshold yields:

$$
D_c(t) = \lambda_c e^{H_0 t} e^{-t/\tau_q} = \lambda_c \exp\biggl(\frac{H_0 \tau_q - 1}{\tau_q}t\biggr).
$$

For small $H_0\tau_q\ll1$, approximate original form with linearization; more generally use the exponential form above.

## 3. Equation & Parameters

1. **Coherence-Distance**:

   $$
   D_c(t) = \lambda_c \exp\Bigl(\tfrac{H_0 t}{\tau_q}\Bigr).
   $$
2. **Threshold condition**: decoherence when $t\approx \tau_q$.
3. **Parameter definitions**:

   * $\lambda_c$ (m)
   * $H_0$ (s$^{-1}$)
   * $\tau_q$ (s)

## 4. Implications & Scaling Limits

* **Terrestrial networks**: $H_0 t\approx10^{-18}$ negligible; local designs unaffected.
* **Interplanetary/QPS**: for $t$ \~ hours, exponential factor \~1; still feasible.
* **Interstellar/Cosmological**: over $t\sim10^{17}$ s, factor huge; coherence distance grows or decays depending on sign of $H_0/\tau_q-1)$ but practical coherence lost far before.

## 5. Experimental Tests

1. **Variable separation tests**: measure entanglement fidelity for photon pairs sent over increasing distances and time delays.
2. **High-coherence systems**: superconducting qubits with $\tau_q\sim1$ s to probe slight expansion effects.
3. **Space-based platforms**: satellite-to-ground entanglement experiments to detect minute exponential scaling.

## 6. Future Extensions

* Incorporate dynamic $H(t)$ for early universe epochs.
* Combine with gravitational potential variations for local curvature corrections.
* Generalize to multiqubit networks and network topology effects.

