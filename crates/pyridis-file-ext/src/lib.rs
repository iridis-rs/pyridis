pub(crate) mod plugin;

pub mod prelude {
    pub use crate::plugin::*;

    pub use pyridis_node::{self, prelude::*};

    pub(crate) use thirdparty::*;

    pub mod thirdparty {
        pub use iridis_file_ext::prelude as ird;
    }
}
