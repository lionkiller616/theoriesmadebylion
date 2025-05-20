# Ambient EM Harvesting

## Concept

Ambient Electromagnetic (EM) Harvesting via metasurfaces is an innovative approach to convert background electromagnetic radiation—such as Wi-Fi, cellular, and radio frequency (RF) signals—into usable direct current (DC) electrical power. This is achieved through engineered metasurfaces that utilize nonlinear rectification mechanisms, offering a substantial increase in efficiency over conventional RF harvesters.

## Design and Details

* **Metasurface Array**: The harvester is composed of subwavelength metallic or dielectric elements arranged in a planar array to absorb specific frequency bands.
* **Nonlinear Rectification Layer**: Each unit cell incorporates a nonlinear component, such as a Schottky diode or tunneling junction, which rectifies the high-frequency AC signal into DC.
* **Resonant Tuning**: Structures are tuned to resonate at 2.4 GHz (Wi-Fi), 900 MHz (cellular), and other common EM bands.
* **DC Collection Grid**: Rectified power from each unit is collected via a network of conductive paths and stored in a capacitor or battery.

## Theoretical Foundation

### Incident Power Density

The power density $P_{inc}$ of ambient EM waves:

$$
P_{inc} = \frac{1}{2} \text{Re}(E \times H^*) = \frac{|E|^2}{2Z_0}
$$

Where:

* $E$: Electric field amplitude
* $H$: Magnetic field amplitude
* $Z_0$: Free-space impedance (≈ 377 Ω)

### Rectified Power Output

For a nonlinear rectifier:

$$
P_{DC} = \eta \cdot P_{AC}
$$

Where:

* $\eta$: Rectification efficiency
* $P_{AC}$: Incident RF power absorbed

### Nonlinear Rectification (Schottky Diode Model)

The diode current-voltage behavior:

$$
I(V) = I_s \left( e^{\frac{qV}{nkT}} - 1 \right)
$$

Where:

* $I_s$: Saturation current
* $q$: Electron charge
* $n$: Ideality factor
* $k$: Boltzmann constant
* $T$: Temperature

DC voltage across the load:

$$
V_{DC} = \frac{1}{T} \int_0^T V(t) \cdot \Theta(V(t)) dt
$$

$\Theta$: Rectification function (nonlinear response)

## Logical Workflow

1. **Absorption**: Metasurface elements resonate with incoming EM waves.
2. **AC-to-DC Conversion**: Nonlinear components rectify oscillating currents into a unidirectional flow.
3. **Aggregation**: Individual DC outputs are collected into a storage element.
4. **Load Delivery**: Stored energy powers low-power electronic devices.
5. **Adaptive Tuning**: Variable load or impedance matching circuits optimize energy capture.

## Predicted Efficiency

* **Target Frequency**: 2.4 GHz (Wi-Fi band)
* **Conversion Efficiency**: Theoretical models predict \~40% under optimized impedance matching and material selection.
* **Benchmark**: Current state-of-the-art RF harvesters achieve 10–15% efficiency.

## Applications

* Battery-free IoT sensors
* Energy-scavenging wearables
* Wireless surveillance systems
* Self-powered smart home devices
* Remote environmental monitors

