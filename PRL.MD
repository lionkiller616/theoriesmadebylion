# Poly-Resonant Logic

## 1. Concept & Overview

PRL replaces discrete voltage levels (0 V/1 V) with **resonant modes** of LC tanks. Each logical variable is encoded as an oscillator that can ring at one of several distinct frequencies:

* **Logical alphabet**:

  $$
    \mathcal{L} = \{f_0,\,f_1,\,(f_2,\dots)\}
  $$

  where $f_0$ represents “0”, $f_1$ represents “1”, etc.

* **State maintenance**:
  Once set to $f_n$, each tank requires only the small sustaining energy to overcome losses—no DC bias is needed.

* **Interconnect**:
  Tanks couple through mutual inductance or capacitive networks; energy only transfers when resonance conditions match.

---

## 2. Single-Tank Dynamics

An isolated LC tank obeys:

$$
L\,\frac{d^2 q}{dt^2} + R\,\frac{dq}{dt} + \frac{q}{C} = 0
$$

where

* $L$ = inductance
* $C$ = capacitance
* $R$ = equivalent series resistance
* $q(t)$ = charge on the capacitor

The undamped natural frequency and quality factor are:

$$
\omega_0 = \frac{1}{\sqrt{LC}}, 
\quad f_0 = \frac{\omega_0}{2\pi},
\quad Q = \frac{\omega_0 L}{R}.
$$

To support multiple frequencies, one can either switch $C$ (e.g. with a varactor) or switch in/out discrete capacitors/inductors.

---

## 3. Coupled-Tank Interaction & Logic Gates

### 3.1. Two-Tank Coupling

When two LC tanks ($L_1,C_1$ and $L_2,C_2$) share mutual inductance $M$, their equations become:

$$
\begin{cases}
L_1 \ddot{q}_1 + R_1 \dot{q}_1 + \tfrac{q_1}{C_1} + M \ddot{q}_2 = 0,\\
L_2 \ddot{q}_2 + R_2 \dot{q}_2 + \tfrac{q_2}{C_2} + M \ddot{q}_1 = 0.
\end{cases}
$$

Define coupling coefficient
$\displaystyle k = \frac{M}{\sqrt{L_1L_2}}$.
Energy transfer is maximized when the two tanks share the same $\omega_0$.

### 3.2. AND Gate

**Inputs**: two tanks $A$ and $B$, each set to either $f_0$ or $f_1$.
**Output**: tank $Y$.

* **Design**: $Y$ is only coupled (via $M$) strongly when both $A$ & $B$ oscillate at $f_1$.
* **Mechanism**: when $f_A=f_B=f_1$, their combined fields reinforce at $f_1$, pumping energy into $Y$’s $f_1$ mode. Otherwise, no net drive exists.

|  A  |  B  | Y (selective drive at $f_1$) |
| :-: | :-: | :--------------------------: |
|  0  |  0  |               0              |
|  0  |  1  |               0              |
|  1  |  0  |               0              |
|  1  |  1  |               1              |

### 3.3. OR Gate

* **Design**: $Y$ couples separately to $A$ and $B$ at frequency $f_1$.
* **Mechanism**: if *either* $A$ or $B$ oscillates at $f_1$, $Y$ receives a drive pulse at $f_1$ and rings up.

|  A  |  B  |  Y  |
| :-: | :-: | :-: |
|  0  |  0  |  0  |
|  0  |  1  |  1  |
|  1  |  0  |  1  |
|  1  |  1  |  1  |

---

## 4. Encoding & Decoding

* **Write (encode)**: briefly inject energy at the target frequency through a small driver coil or capacitive kick.
* **Hold**: a low-power sustaining amplifier compensates for losses ($\propto1/Q$).
* **Read (decode)**: sense the tank voltage or current; perform a band-pass filter + envelope detector at each $f_n$ to resolve the logical value.

---

## 5. Error Correction & Noise Immunity

### 5.1. Phase-Locking

By phase-locking each tank to a reference clock at its chosen $f_n$, one can correct drift. A simple PLL adds:

$$
\dot\theta = \omega_\text{ref} - K\,\sin(\theta_\text{tank}-\theta_\text{ref}),
$$

forcing the tank phase/frequency back to $\omega_\text{ref}=2\pi f_n$.

### 5.2. Redundancy & Voting

* **Tri-tank encoding**: use three parallel LC tanks per bit; majority-vote on their decoded outputs to correct single-tank failures.

---

## 6. Scaling & Integration

* **Ternary or higher-radix logic**: simply add more resonant frequencies ($f_2,f_3,\dots$).
* **RF interconnect**: tanks can drive small antennas—logic values can be wirelessly broadcast or combined in free space.
* **3D integration**: multilayer coils allow hundreds of tanks per mm² with minimal crosstalk if carefully shielded.

