extern crate crossbeam;

struct Engine {
    config: EngineConfig,
    state: EngineState,

}

// TODO allow for multiple
impl Engine {
    fn new(
        config:EngineConfig,
        state:EngineState
    ) -> Self {
        let engine = Engine {
            config,
            ts_store: new KDBStore(
                config.kdb_host,
                config.kdb_port,
                config.kdb_pass,
                config.kdb_user,
                prepared_query,
                ingress_channel,
                obs_channel
            ),
            ls_cache: new BitmexState(),
            ingress_workers: Vec::new(
                new BitmexWorker(ingress_channel),
            ),
            exchange_client: new BitmexClient(),
            inference_agent: new HFTAgent(),
            ingress_channel: ingress_channel,
            update_channel: obs_channel
        }

        engine.initialize();
    }

    // Initialize creates tables for each ingress agent
    // in kdb as well as initializes the exchange client
    // with rest requests if run server is set to true.
    // If run agent is set to true it will also check that
    // the agent successfully produces an action that conforms
    // to the action config.
    fn initialize(&self) {

    }

    // This combines all components for data ingress
    // and subsequent serving into one class and makes
    // this state available via grpc and streaming updates
    // to the client implementation of the same class.
    // This allows for the flexible integration of both
    // classes in run as well as seperate integrations
    // that allow for seperate execution and data ingress.
    fn run_server(&self) {
        self.ts_store.run_ingress();

        for worker in self.ingress_workers {
            worker.run();
        }

        // This runs a query at a recurring time
        // interval depending on
        self.ts_store.gen_observations()
    }

    // This listens for updates from the obs channel and
    // subsequently get predictions from the agent afterwards
    // the predicton which conforms to a set action type in
    // the config. This action is then morphed into a vector
    // representation of actions before being used to generate
    // and by extension place orders.
    fn run_agent(&self) {

    }
}
