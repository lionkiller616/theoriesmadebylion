# Entropic Encryption Systems

## 1. Introduction

Entropic Encryption Systems encode information within controlled thermal noise patterns, leveraging the inherent randomness of temperature-driven fluctuations to secure data. By modulating and sampling noise entropy in physical channels, these systems provide dynamic, tamper-resistant encryption that is inherently resistant to external interception or reconstruction.

## 2. Concept and Motivation

* **Thermal Noise as Carrier**: Thermal (Johnson–Nyquist) noise in resistive elements is exploited as a high-entropy medium for encoding bits.
* **Dynamic Key Generation**: Keys are generated on-the-fly from noise realizations, eliminating stored keys and reducing attack surfaces.
* **Physical Layer Security**: Encryption at the physical layer complements traditional algorithms by embedding data directly in physical fluctuations.
* **Tamper Evident**: Any external probing perturbs the noise statistics, alerting legitimate endpoints to potential breaches.

## 3. Underlying Theory

### 3.1 Johnson–Nyquist Noise

A resistor of resistance $R$ at temperature $T$ produces voltage noise with power spectral density:

$$
S_V(f) = 4 k_B T R,
$$

where $k_B$ is Boltzmann’s constant. The time-domain noise voltage $v(t)$ is a zero-mean Gaussian process with variance:

$$
\sigma^2 = \langle v^2 \rangle = 4 k_B T R \Delta f,
$$

for bandwidth $\Delta f$.

### 3.2 Entropy and Information Capacity

The differential entropy of a Gaussian noise source of variance $\sigma^2$ is:

$$
H = \tfrac{1}{2} \ln(2 \pi e \sigma^2).
$$

Maximum information that can be extracted per sample is bounded by channel capacity:

$$
C = \tfrac{1}{2} \log_2\bigl(1 + \tfrac{P}{N_0 f_s}\bigr),
$$

where $P$ is signal power, $N_0 = 2 k_B T R$ noise spectral density, and $f_s$ sampling rate.

### 3.3 Controlled Noise Modulation

By actively heating or cooling the resistor (or using Peltier elements), temperature $T(t)$ is modulated according to a secret key stream. The resulting noise variance $\sigma^2(t)$ carries the encryption waveform:

$$
\sigma^2(t) = 4 k_B R \Delta f \, T(t).
$$

Data bits are encoded by synchronized sampling of high-entropy intervals.

## 4. System Design

### 4.1 Physical Layer Transmitter

1. **Resistive Element**: Precision resistor mounted on a thermal actuator.
2. **Thermal Modulator**: Peltier device driven by control electronics to impose temperature profile $T(t)$.
3. **Noise Sampler**: High-speed ADC captures noise voltages in defined time windows.
4. **Bit Encoder**: Maps sampled noise statistics to bit sequences via quantization thresholds.

### 4.2 Physical Layer Receiver

1. **Thermal Tracking**: Mirror Peltier control to match $T(t)$ profile or use out-of-band coordinate channel.
2. **Noise Sampler**: Identical ADC and quantizer setup.
3. **Bit Decoder**: Applies the same thresholds to reconstruct transmitted bits.
4. **Error Correction**: Forward error correction codes mitigate sampling mismatches.

## 5. Key Equations Summary

* Noise PSD: $S_V(f)=4 k_B T R$
* Noise variance: $\sigma^2=4 k_B T R \Delta f$
* Entropy: $H=\tfrac12 \ln(2 \pi e \sigma^2)$
* Capacity: $C=\tfrac12 \log_2(1+P/(N_0 f_s))$
* Time-varying variance: $\sigma^2(t)=4 k_B R \Delta f\, T(t)$

## 6. Logical Workflow

1. **Initialization**: Agree on timing, thermal modulation pattern, and quantization scheme.
2. **Modulation**: Transmitter imposes secret temperature waveform $T(t)$.
3. **Sampling**: Both ends sample noise and quantize to bits in synchronized slots.
4. **Error Correction & Verification**: Apply ECC and verify integrity.
5. **Data Exchange**: Use recovered noise-derived bitstream as one-time pad for higher-layer encryption.

## 7. Security Analysis

* **Entropy Source**: Inherent physical randomness resists software-based attacks.
* **Tamper Detection**: External perturbations shift noise statistics, detectable via statistical tests.
* **Key Renewal**: Continuous noise supply allows frequent key updates.
* **Side-Channel Resistance**: Physical embedding reduces leakage compared to logic-level encryption.

## 8. Conclusion

Entropic Encryption Systems merge thermodynamics and information theory to secure data at the physical layer. With precise thermal control and sampling, they offer a novel, high-entropy channel for robust, tamper-evident encryption in critical applications.

## 9. Performance Metrics

* **Entropy Rate**: Bits of entropy generated per second, $R_H = H \times f_s$, where $H$ is per-sample entropy and $f_s$ the sampling rate.
* **Key Generation Rate**: $R_K = f_s \times \log_2(N_q)$, with $N_q$ quantization levels per sample.
* **Bit Error Rate (BER)**: Probability of mismatched bits between transmitter and receiver, $\mathrm{BER} = \Pr(b_t \neq b_r)$.
* **Signal-to-Noise Ratio (SNR)**: $\mathrm{SNR} = P/(N_0 f_s)$, impacting capacity and error performance.
* **Thermal Modulation Bandwidth**: Maximum modulation frequency $f_m$ of temperature, limited by Peltier response time and thermal time constant.
* **Energy per Bit**: $E_b = P_{th}/R_K$, where $P_{th}$ is power consumed in thermal modulation.
