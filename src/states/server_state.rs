use amethyst::{
    prelude::*,
};

use crate::resources::{IO, AppConfig, MapList, LifeformList};

pub struct ServerState{
    pub config: AppConfig,
}

impl SimpleState for ServerState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let io = IO::new();
        
        // Load in all the maps in the world
        let mut maps = MapList::new();
        let mut lifeforms = LifeformList::new();
        
        // Load in all the maps
        maps.add("resources/maps/town.tmx".to_string());

        world.insert(self.config.clone());
        world.insert(io);
        world.insert(maps);
        world.insert(lifeforms);
    }
}
