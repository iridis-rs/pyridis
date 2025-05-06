pub mod prelude {
    pub use pyridis_file_ext::{self, prelude::*};

    pub use thirdparty::*;

    pub mod thirdparty {
        pub use iridis::prelude as ird;
    }
}
