extern crate crossbeam;

struct Engine {
    config: EngineConfig,
    state: EngineState,

}

impl Engine {
    fn new(
        config:EngineConfig,
        state:EngineState
    ) -> Self {
        Engine {
            config,
            state:StateWorker::new(

            ),


        }
    }
}
