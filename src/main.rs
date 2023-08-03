use planet_sim::engine;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut engine = engine::Engine::new()?;
    pollster::block_on(engine.init())?;

    engine.begin_loop()?;

    Ok(())
}
