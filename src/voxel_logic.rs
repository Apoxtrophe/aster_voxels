fn simulation_tick_system(time: Res<Time>, mut last_update: Local<f64>, ...) {
    if time.seconds_since_startup() - *last_update >= 0.1 {
        // Update game logic here

        *last_update = time.seconds_since_startup();
    }
}