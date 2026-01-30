use cubecl::prelude::*;

// This function will be executed in parallel on the GPU/CPU
// Lookup the variable ABSOLUTE_POS to get started!
#[cube(launch_unchecked)]
fn compute_bbf<F: Float>(max_iter: u32, vec_size: u32, output: &mut Array<f32>) {

    if ABSOLUTE_POS < max_iter as usize {
        let mut id = 0;
        for i in 0..vec_size {
            let k = id as f32;
            let mut power = 1.0;
            for _ in 0..id {
                power *= 16.0;
            }
            let val = 1. / power;
            let denom = 8. * k;
            let sum = 4. / (denom + 1.) - 2. / (denom + 4.) - 1. / (denom + 5.) - 1. / (denom + 6.);
            output[i as usize] = val * sum;
            id += 1;
        }
    }
}

fn final_sum(arr: &[f32], size: usize) -> f64 {
    let mut sum = 0.0;
    for i in 0..size {
        let e = arr[i];
        println!("{e}");
        sum += arr[i] as f64;
    }
    sum
}

pub fn launch<R: Runtime>(device: &R::Device) {
    let client = R::client(device);

    let vectorization = 2usize;
    let max_iter = 4;//1000_00_000u32;

    let output_handle = client.empty(max_iter as usize * core::mem::size_of::<f32>());

    unsafe {
        compute_bbf::launch_unchecked::<f32, R>(
            &client,
            CubeCount::Static(1, 1, 1),
            CubeDim::new_1d(max_iter as u32 / vectorization as u32),
            ScalarArg::new(max_iter),
            ScalarArg::new(vectorization as u32),
            ArrayArg::from_raw_parts::<f32>(&output_handle, max_iter as usize, vectorization)
        )
        .unwrap()
    };

    let arr_pointer = client.read_one(output_handle);

    let arr = f32::from_bytes(&arr_pointer);

    let result = final_sum(arr, vectorization);

    println!("Answer => {result}",);
}

fn launch_cubcl() {
    #[cfg(feature = "cuda")]
    {
        print!("Launching with CUDA runtime...\n");
        launch::<cubecl::cuda::CudaRuntime>(&Default::default());
    }

    #[cfg(feature = "wgpu")]
    {
        print!("Launching with WGPU runtime...\n");
        launch::<cubecl::wgpu::WgpuRuntime>(&Default::default());
    }

    #[cfg(feature = "cpu")]
    {
        print!("Launching with CPU runtime...\n");
        launch::<cubecl::cpu::CpuRuntime>(&Default::default());
    }
}

fn main() {
    println!("Bailey-Borwein-Plouffe with cubecl!");
    launch_cubcl();
}
