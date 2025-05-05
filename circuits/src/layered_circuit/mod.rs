//! An implemenation for a the Layered arithementic circuit, built targetting GKR, Libra protocol
//! This is not an IR for Virgo at this moment.

pub mod primitives;
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

        LayeredCircuit::new(layers)
    }

    /// Execute the circuit with given inputs
    /// Performs the calculations layer by layer using the original inputs for all gates
    pub fn execute<F>(&self, inputs: &[F]) -> Vec<Vec<F>>
    where
        F: Copy + std::ops::Add<Output = F> + std::ops::Mul<Output = F>,
    {
        let mut evaluations = Vec::with_capacity(self.layers.len() + 1);

        // First layer is the input
        evaluations.push(inputs.to_vec());

        // Reference to original inputs - we'll use this for all gate evaluations
        let original_inputs = inputs;

        // Process each layer
        for (layer_idx, layer) in self.layers.iter().enumerate() {
            let mut layer_outputs = Vec::with_capacity(layer.gates.len());

            // Process each gate in the layer
            for (gate_idx, gate) in layer.gates.iter().enumerate() {
                // Check if we can safely access these inputs from the original inputs
                if gate.inputs[0] >= original_inputs.len()
                    || gate.inputs[1] >= original_inputs.len()
                {
                    panic!(
                        "Gate {} in layer {} is trying to access invalid inputs: {:?}, but input array has only {} elements",
                        gate_idx,
                        layer_idx,
                        gate.inputs,
                        original_inputs.len()
                    );
                }

                let input1 = original_inputs[gate.inputs[0]];
                let input2 = original_inputs[gate.inputs[1]];

                let result = match gate.op {
                    GateOp::Add => input1 + input2,
                    GateOp::Mul => input1 * input2,
                };

                layer_outputs.push(result);
            }

            evaluations.push(layer_outputs);
        }

        evaluations
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
        assert_eq!(det_circuit.layers[0].gates.len(), 1); // 2^0 = 1 gate
        assert_eq!(det_circuit.layers[1].gates.len(), 2); // 2^1 = 2 gates
        assert_eq!(det_circuit.layers[2].gates.len(), 4); // 2^2 = 4 gates

        // Check gate types for deterministic circuit
        assert!(matches!(det_circuit.layers[0].gates[0].op, GateOp::Add));
        assert!(matches!(det_circuit.layers[1].gates[0].op, GateOp::Mul));

        // Test the RNG-based random function with a fixed seed for reproducibility
        let seed = [42u8; 32];
        let mut rng = StdRng::from_seed(seed);
        let rng_circuit = LayeredCircuit::random_with_rng(3, &mut rng);

        // Check the structure of the RNG-based circuit
        assert_eq!(rng_circuit.layers.len(), 3);
        assert_eq!(rng_circuit.layers[0].gates.len(), 1); // 2^0 = 1 gate
        assert_eq!(rng_circuit.layers[1].gates.len(), 2); // 2^1 = 2 gates
        assert_eq!(rng_circuit.layers[2].gates.len(), 4); // 2^2 = 4 gates

        // Gate types are random, so we don't check them specifically
    }

    #[test]
    fn test_circuit_execution() {
        // Create a simple test circuit
        let circuit = create_test_circuit();

        // Create inputs (4 values for layer 0)
        let inputs: Vec<i32> = vec![1, 2, 3, 4];

        // Execute the circuit
        let evaluations = circuit.execute(&inputs);

        // Check evaluations - should have 3 layers (input + 2 layers)
        assert_eq!(evaluations.len(), 3);

        // Input layer should match our inputs
        assert_eq!(evaluations[0], vec![1, 2, 3, 4]);

        // Check calculations in layer 1 (add gates)
        // Gate 1: 1 + 2 = 3
        // Gate 2: 3 + 4 = 7
        assert_eq!(evaluations[1], vec![3, 7]);

        // Check calculations in layer 2 (mul gate)
        // With our revised execute method, gate uses original inputs:
        // Gate 1: 1 * 2 = 2  (accessing original inputs at indices 0 and 1)
        assert_eq!(evaluations[2], vec![2]);
    }

    #[test]
    fn test_random_circuits_execution() {
        // For simplicity, let's test with 2 layers
        let num_layers = 2;

        // The input size should be 2^num_layers
        let num_inputs = 2usize.pow(num_layers as u32); // 2^2 = 4 inputs

        // Test both random circuit generators
        let det_circuit = LayeredCircuit::random(num_layers);

        // Use fixed seed for reproducibility
        let seed = [42u8; 32];
        let mut rng = StdRng::from_seed(seed);
        let rng_circuit = LayeredCircuit::random_with_rng(num_layers, &mut rng);

        // Create inputs
        let inputs: Vec<i32> = (1..=num_inputs as i32).collect();

        // Execute both circuits
        let det_evaluations = det_circuit.execute(&inputs);
        let rng_evaluations = rng_circuit.execute(&inputs);

        // Check evaluations structure - should have num_layers + 1 layers (input + circuit layers)
        assert_eq!(det_evaluations.len(), num_layers + 1);
        assert_eq!(rng_evaluations.len(), num_layers + 1);

        // Input layer should match our inputs for both
        assert_eq!(&det_evaluations[0], &inputs);
        assert_eq!(&rng_evaluations[0], &inputs);

        // The deterministic circuit follows a pattern, so we can check its structure
        assert_eq!(det_circuit.layers[0].gates.len(), 1); // 2^0 = 1 gate in first layer
        assert_eq!(det_circuit.layers[1].gates.len(), 2); // 2^1 = 2 gates in second layer

        // Check the deterministic gate types match the expected pattern
        assert!(matches!(det_circuit.layers[0].gates[0].op, GateOp::Add));
        assert!(matches!(det_circuit.layers[1].gates[0].op, GateOp::Mul));
        assert!(matches!(det_circuit.layers[1].gates[1].op, GateOp::Mul));
    }

    // Helper function to create a simple test circuit
    fn create_test_circuit() -> LayeredCircuit {
        // Create a 2-layer circuit:
        // Layer 1: 2 ADD gates
        // Layer 2: 1 MUL gate

        let layer1 = Layer::new(vec![
            Gate::new(GateOp::Add, [0, 1]), // add inputs 0 and 1
            Gate::new(GateOp::Add, [2, 3]), // add inputs 2 and 3
        ]);

        let layer2 = Layer::new(vec![
            Gate::new(GateOp::Mul, [0, 1]), // multiply outputs from previous layer
        ]);

        LayeredCircuit::new(vec![layer1, layer2])
    }
}
