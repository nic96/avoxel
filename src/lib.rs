pub mod prelude;

mod default_plugins;
pub use default_plugins::*;

pub mod blocks {
    pub use avoxel_blocks::*;
    #[cfg(feature = "rendering")]
    pub use avoxel_rendering::prelude::*;
}

pub mod chunk {
    pub use avoxel_chunk::*;
}

pub mod math {
    pub use avoxel_math::*;
}

pub mod physics {
    pub use avoxel_physics::*;
}

#[cfg(feature = "rendering")]
pub mod rendering {
    pub use avoxel_rendering::prelude::*;
}
