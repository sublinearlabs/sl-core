//! An implemenation for a the Layered arithementic circuit, built targetting GKR, Libra protocol
//! This is not an IR for Virgo at this moment.
pub mod circuit;
pub mod grk;
pub mod primitives;
pub mod utils;

use primitives::{Gate, GateOp, Layer};
use rand;

/// Layered Circuit, a layered sturcture of gate composites
#[derive(Debug, Clone)]
pub struct LayeredCircuit {
    /// The curcuit is just a vec of circuit-layers
    pub layers: Vec<Layer>,
}

impl LayeredCircuit {
    /// util fn for creating new circuits
    pub fn new(layers: Vec<Layer>) -> Self {
        Self { layers }
    }

    /// Generate a random circuit with a specific number of layers
    pub fn random(num_of_layers: usize) -> Self {
        let mut layers = Vec::new();

        for layer_index in 0..num_of_layers {
            let mut layer = Vec::new();
            let number_of_gates = 2usize.pow(layer_index as u32);
            let number_of_inputs = 2usize.pow((layer_index + 1) as u32);

            for gate_index in 0..number_of_gates {
                let input_1 = (gate_index * 2) % number_of_inputs;
                let input_2 = (gate_index * 2 + 1) % number_of_inputs;
                let g_type = if layer_index % 2 == 0 {
                    GateOp::Add
                } else {
                    GateOp::Mul
                };
                layer.push(Gate::new(g_type, [input_1, input_2]));
            }

            layers.push(Layer::new(layer));
        }

        layers.reverse();
        LayeredCircuit::new(layers)
    }

    /// Generate a random circuit using a provided random number generator
    pub fn random_with_rng<R: rand::Rng>(num_of_layers: usize, rng: &mut R) -> Self {
        let mut layers = Vec::new();

        for layer_index in 0..num_of_layers {
            let mut layer = Vec::new();
            let number_of_gates = 2usize.pow(layer_index as u32);
            let number_of_inputs = 2usize.pow((layer_index + 1) as u32);

            for _ in 0..number_of_gates {
                // Get random inputs from available inputs for this layer
                let input_1 = rng.gen_range(0..number_of_inputs);
                let input_2 = rng.gen_range(0..number_of_inputs);

                // Randomly choose gate type
                let g_type = if rng.gen_bool(0.5) {
                    GateOp::Add
                } else {
                    GateOp::Mul
                };

                layer.push(Gate::new(g_type, [input_1, input_2]));
            }

            layers.push(Layer::new(layer));
        }

        layers.reverse();
        LayeredCircuit::new(layers)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;
    use rand::rngs::StdRng;

    #[test]
    fn test_random_circuit_generation() {
        // Test the deterministic random function
        let det_circuit = LayeredCircuit::random(3);

        // Check the structure of the deterministic circuit
        assert_eq!(det_circuit.layers.len(), 3);
        assert_eq!(det_circuit.layers[0].gates.len(), 4); // 2^0 = 4 gate
        assert_eq!(det_circuit.layers[1].gates.len(), 2); // 2^1 = 2 gates
        assert_eq!(det_circuit.layers[2].gates.len(), 1); // 2^2 = 1 gates

        // Check gate types for deterministic circuit
        assert!(matches!(det_circuit.layers[1].gates[0].op, GateOp::Mul));
        assert!(matches!(det_circuit.layers[0].gates[0].op, GateOp::Add));

        // Test the RNG-based random function with a fixed seed for reproducibility
        let seed = [42u8; 32];
        let mut rng = StdRng::from_seed(seed);
        let rng_circuit = LayeredCircuit::random_with_rng(3, &mut rng);

        // Check the structure of the RNG-based circuit
        assert_eq!(rng_circuit.layers.len(), 3);
        assert_eq!(rng_circuit.layers[0].gates.len(), 4); // 2^0 = 4 gate
        assert_eq!(rng_circuit.layers[1].gates.len(), 2); // 2^1 = 2 gates
        assert_eq!(rng_circuit.layers[2].gates.len(), 1); // 2^2 = 1 gates

        // Gate types are random, so we don't check them specifically
    }
}
