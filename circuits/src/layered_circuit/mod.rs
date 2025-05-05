//! An implemenation for a the Layered arithementic circuit, built targetting GKR, Libra protocol
//! This is not an IR for Virgo at this moment.

pub mod primitives;
use primitives::{Layer, Gate, GateOp};
use rand;

/// Layered Circuit, a layered sturcture of gate composites
#[derive(Debug, Clone)]
pub struct LayeredCircuit {
    /// The curcuit is just a vec of circuit-layers
    pub layers: Vec<Layer>
}



impl LayeredCircuit {
    /// util fn for creating new circuits 
    pub fn new(layers: Vec<Layer>) -> Self {
        Self {
            layers
        }
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
}