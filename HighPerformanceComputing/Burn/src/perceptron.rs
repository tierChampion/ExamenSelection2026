use burn::backend::NdArray;
use burn::tensor::Tensor;

type Backend = NdArray<f32>;

pub struct Perceptron {
    weights: Tensor<Backend, 1>,
    learning_rate: f32,
}

impl Perceptron {
    /// Create a new perceptron with n inputs
    pub fn new(n: usize) -> Self {
        // Initialize weights randomly between -1 and 1
        let device = Default::default();
        let random_weights: Vec<f32> = (0..n).map(|_| rand::random::<f32>() * 2.0 - 1.0).collect();

        let weights = Tensor::<Backend, 1>::from_floats(random_weights.as_slice(), &device);

        Self {
            weights,
            learning_rate: 0.01,
        }
    }

    // TODO: Implement feed_forward method
    pub fn feed_forward(&self, inputs: &[f32]) -> i32 {
        let device = Default::default();

        let input_tensor = Tensor::<Backend, 1>::from_floats(inputs, &device);

        // Compute dot product: sum of (weights * inputs)
        let x = (self.weights.clone() * input_tensor).sum();

        // Return the sum passed through the activation function
        Perceptron::activate(x.into_scalar())
    }

    // TODO: Implement the activation function
    //       You can try different ones such as step, sigmoid, or Sign
    fn activate(s: f32) -> i32 {
        // 0 for
        (1.0 / (1.0 + (-s).exp())) as i32
    }

    /// TODO: Implement the training method
    pub fn train(&mut self, inputs: &[f32], desired: i32) {
        let guess = self.feed_forward(inputs);
        let error = (desired - guess) as f32;

        // Update weights: w += learning_rate * error * input
        let device = Default::default();

        let input_tensor = Tensor::<Backend, 1>::from_floats(inputs, &device);
        let adjustment = self.learning_rate * error * input_tensor;

        self.weights = self.weights.clone() + adjustment;
    }

    /// Get the current weights as a Vec for display
    pub fn get_weights(&self) -> Vec<f32> {
        self.weights.clone().into_data().to_vec().unwrap()
    }
}
