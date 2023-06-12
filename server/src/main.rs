

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let mut state = State {
    //     ships: vec![],
    //     planets: vec![
    //         Planet {
    //             heading: Radians::new(0.0),
    //             mass: 100000.0,
    //             position: array![100.0, 100.0],
    //             radius: 10.0,
    //             spin: Radians::new(0.0),
    //             velocity: array![20.0, 0.0],
    //         },
    //         Planet {
    //             heading: Radians::new(0.0),
    //             mass: 100000.0,
    //             position: array![100.0, 200.0],
    //             radius: 10.0,
    //             spin: Radians::new(0.0),
    //             velocity: array![-20.0, 0.0],
    //         },
    //         Planet {
    //             heading: Radians::new(0.0),
    //             mass: 100000.0,
    //             position: array![300.0, 100.0],
    //             radius: 10.0,
    //             spin: Radians::new(0.0),
    //             velocity: array![-20.0, 0.0],
    //         },
    //     ],
    //     inputs: Inputs::default(),
    // };

    // let mut physics = NBodyPhysics::new(None, &state);

    // let mut steps_per_update: u32 = 2;
    // let mut prev_time = std::time::Instant::now();
    // loop {
    //     let now = std::time::Instant::now();
    //     let dt = now.duration_since(prev_time).as_secs_f32();
    //     prev_time = now;

    //     let fps = (if dt > 0.0 { 1.0 / dt } else { 0.0 }) as i32;

    //     // Tune the number of steps to try to hit the target SPU
    //     steps_per_update = (steps_per_update as i32 + (fps - TARGET_STEPS_PER_UPDATE)).clamp(10, 1000) as u32;
    //     println!("{}", steps_per_update);

    //     physics.step(&mut state, dt as f64, steps_per_update);
    // }

    let addr = "[::1]:50051".parse()?;
    let greeter = MyGreeter::default();

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
