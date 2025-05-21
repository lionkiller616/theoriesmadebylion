# Radiative Cooling Panels: Spectrally Selective Emitters Dumping Heat to the Cold Sky

This document describes the concept, design, theoretical principles, governing equations, operational logic, and practical considerations for passive radiative cooling panels that achieve sub-ambient cooling by selectively emitting infrared radiation to the cold outer atmosphere (sky).

---

## 1. Concept and Motivation

* **Objective**: Achieve passive cooling below ambient temperature by radiating thermal energy through the atmospheric transparency window (8–13 µm) into cold space (\~3 K).
* **Applications**: Building temperature regulation, outdoor refrigeration, solar cell temperature control, vehicle cooling, thermal management of electronics.
* **Advantages**:

  * **Energy-free operation** using no electricity or moving parts.
  * **Continuous cooling** day and night when designed for solar reflection.
  * **Scalable** for rooftop panels or integrated surfaces.

---

## 2. Panel Architecture

### 2.1. Spectrally Selective Emitter

* **Top layer**: Dielectric stack or metamaterial engineered to reflect solar spectrum (0.3–2.5 µm) and emit strongly in mid-IR (8–13 µm).
* **Emitter materials**: Silicon dioxide, hafnium dioxide, polymers (e.g., PDMS), photonic crystals.
* **Thickness design**: Quarter-wave layers tuned to target IR band.

### 2.2. Substrate and Backing

* **Low-thermal-conductivity substrate**: Polymeric foam or aerogel to minimize conductive heat gain from ambient.
* **Metallic back reflector**: Aluminum or silver layer to reflect unabsorbed solar radiation and IR back toward sky.

### 2.3. Encapsulation

* **Protective coating**: UV-stable, hydrophobic layer to guard against environmental damage.
* **Encased assembly**: Transparent cover (e.g., polyethylene film) that transmits infrared while blocking moisture.

---

## 3. Theoretical Background

### 3.1. Radiative Heat Exchange

Net radiative power per unit area between panel at temperature $T_p$ and sky effective temperature $T_	ext{sky}$:

$$
P_	ext{rad} = \int_0^\infty arepsilon(\lambda)\,I_{bb}(\lambda,T_p)\,d\lambda \;-\; \int_0^\infty arepsilon(\lambda)\,I_{atm}(\lambda,T_	ext{sky})\,d\lambda
$$

* $\varepsilon(\lambda)$: spectral emissivity of panel.
* $I_{bb}(\lambda,T)$: blackbody spectral radiance at wavelength $\lambda$.
* $I_{atm}(\lambda,T_	ext{sky})$: downwelling atmospheric radiance.

### 3.2. Solar and Environmental Loads

Absorbed solar power:

$$
P_	ext{solar} = \int_0^\infty A(\lambda)\,I_\odot(\lambda)\,d\lambda
$$

* $A(\lambda)$: spectral absorptivity (≈1 − reflectivity).
* $I_\odot(\lambda)$: solar irradiance spectrum.

Non-radiative heat gains (convection/conduction):

$$
P_	ext{conv} = h_c (T_a - T_p)
$$

* $h_c$: convective heat transfer coefficient.
* $T_a$: ambient air temperature.

### 3.3. Equilibrium Temperature

At steady state, net power is zero:

$$
P_	ext{rad} - P_	ext{solar} + P_	ext{conv} = 0
$$

Solve for panel temperature $T_p$ giving sub-ambient cooling when $P_	ext{rad} > P_	ext{solar} + |P_	ext{conv}|$.

---

## 4. Governing Equations and Spectral Design

### 4.1. Atmospheric Transmission Window

Atmospheric emissivity/transmissivity $\tau(\lambda)$ must be high in 8–13 µm band. Effective sky radiance:

$$
I_{atm}(\lambda) = \bigl[1 - \tau(\lambda)\bigr] I_{bb}(\lambda,T_a)
$$

### 4.2. Multilayer Reflectance and Emittance

Using transfer-matrix method, reflectivity $R(\lambda)$ of a stack of $N$ layers:

$$
\mathbf{M} = \prod_{i=1}^N \begin{pmatrix} \cos(\delta_i) & j \sin(\delta_i)/n_i \\
 j n_i \sin(\delta_i) & \cos(\delta_i) \end{pmatrix},
$$

with phase thickness $\delta_i = 2\pi n_i d_i / \lambda$. Emissivity $\varepsilon = 1 - R$ (opaque substrate).

### 4.3. Cooling Power Density

Integrated net cooling power:

$$
P_	ext{cool}(T_p) = \int_{\lambda_1}^{\lambda_2} \varepsilon(\lambda) \, \bigl[I_{bb}(\lambda,T_p) - I_{atm}(\lambda)\bigr] \, d\lambda \\
- \int_\text{solar} A(\lambda) I_\odot(\lambda) d\lambda - h_c (T_a - T_p)
$$

Optimizing $\varepsilon(\lambda)$ inside 8–13 µm and minimizing absorptance outside maximizes $P_	ext{cool}$.

---

## 5. Operational Logic Flow

1. **Material Selection**: Choose dielectric materials with appropriate refractive indices.
2. **Layer Design**: Compute layer thicknesses using quarter-wave condition in IR band.
3. **Fabrication**: Deposit multilayer stack and metallic back reflector on substrate.
4. **Installation**: Mount panels facing sky with minimal obstructions; ensure ventilation.
5. **Performance Monitoring**: Measure panel surface temperature and ambient conditions.
6. **Model Validation**: Compare measured cooling power to theoretical predictions; iterate design.

---

## 6. Design Considerations and Challenges

* **Daytime Operation**: Integrate high reflectivity in solar band to avoid heating.
* **Humidity and Clouds**: Atmospheric transmission degrades under high humidity or cloud cover.
* **Wind Effects**: High convection can reduce net cooling power.
* **Durability**: Long-term UV exposure and environmental wear require robust coatings.
* **Scalability and Cost**: Balance performance with manufacturability for large-area deployment.

---

## 7. References

1. Raman, A. P., Anoma, M. A., Zhu, L., Rephaeli, E., & Fan, S. (2014). *Passive Radiative Cooling Below Ambient Air Temperature Under Direct Sunlight*. Nature, 515(7528), 540–544.
2. Gentle, A. R., & Smith, G. B. (2015). *Radiative Heat Pumping from the Earth Using Surface Phonon Resonant Nanophotonics*. Nano Letters, 15(2), 1003–1008.
3. Zhai, Y., et al. (2017). *Scalable-Manufactured Randomized Glass-Polymer Hybrid Metamaterial for Daytime Radiative Cooling*. Science, 355(6329), 1062–1066.

---

*End of Document*
