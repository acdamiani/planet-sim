use planet_sim::win::run;

fn main() {
    pollster::block_on(run());
}
